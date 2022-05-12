use crate::request::Request;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::{json, Value as JsonValue};
use std::thread;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::header::HeaderValue, Message::Text},
    MaybeTlsStream, WebSocketStream,
};

#[derive(Debug, Deserialize)]
struct ConnectResponse {
    url: String,
}

struct DiscordGateway {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    token: String,
    heartbeat_thread: Option<thread::JoinHandle<()>>,
}

impl DiscordGateway {
    const DISPATCH: u8 = 0;
    const HEARTBEAT: u8 = 1;
    const IDENTIFY: u8 = 2;
    const PRESENCE: u8 = 3;
    const VOICE_STATE: u8 = 4;
    const VOICE_PING: u8 = 5;
    const RESUME: u8 = 6;
    const RECONNECT: u8 = 7;
    const REQUEST_MEMBERS: u8 = 8;
    const INVALIDATE_SESSION: u8 = 9;
    const HELLO: u8 = 10;
    const HEARTBEAT_ACK: u8 = 11;
    const GUILD_SYNC: u8 = 12;

    async fn connect(token: &str) -> DiscordGateway {
        let gateway_request = Request::get("/gateway/bot")
            .authorize(token)
            .with_param("v", "9")
            .with_param("encoding", "json");

        let response = gateway_request.call().await.unwrap();

        let url = response.json::<ConnectResponse>().await.unwrap().url;

        let mut client = format!("{}/?v=9&encoding=json", url)
            .into_client_request()
            .unwrap();
        client.headers_mut().append(
            "Authorization",
            HeaderValue::from_str(&format!("Bot {}", token)).unwrap(),
        );

        let res = tokio_tungstenite::connect_async(client).await;
        let (mut ws, _) = res.unwrap();

        let mut result = Self {
            ws,
            token: String::from(token),
            heartbeat_thread: None,
        };

        result.hello().await;

        result
    }

    async fn hello(&mut self) {
        let response = self.ws.next().await.unwrap().unwrap();
        if let Text(response) = response {
            let response: GatewayPayload = serde_json::from_str(&response).unwrap();
            // println!("{:?}", response);
            let heartbeat_interval = response.d.unwrap().as_object().unwrap()["heartbeat_interval"]
                .as_u64()
                .unwrap();
            self.heartbeat_thread = Some(thread::spawn(move || {
                let initial_delay = (heartbeat_interval as f32 * fastrand::f32()) as u64;
                thread::sleep(Duration::from_millis(initial_delay));
                let heartbeat_delay = Duration::from_millis(heartbeat_interval);
                loop {
                    thread::sleep(heartbeat_delay);
                }
            }));
        }
    }

    fn close(&mut self) {
        self.ws.close(None);
    }

    fn identify(&mut self) {
        let payload = json! ({
            "op": Self::IDENTIFY,
            "d": {
                "token": self.token.as_str(),
                "properties": {
                    "$browser": "discord-bot.rs",
                    "$device": "discord-bot.rs"
                },
                "compress": false,
                "v": 9
            }
        });

        // let message = OwnedMessage::Text(payload.to_string());

        // self.ws.send_message(&message);
    }
}

#[derive(Debug, Deserialize)]
struct GatewayPayload {
    op: u8,
    d: Option<JsonValue>,
    s: Option<u32>,
    t: Option<String>,
}

#[cfg(test)]
mod gateway_test {
    use super::*;
    #[tokio::test]
    async fn gateway() {
        let bot_token = dotenv::var("BOT_TOKEN").unwrap();
        let mut ws = DiscordGateway::connect(&bot_token).await;
        ws.close();
    }
}
