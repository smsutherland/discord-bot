use crate::request::Request;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value as JsonValue};
use std::{net::Shutdown, time::Duration};
use tokio::{
    net::TcpStream,
    sync::broadcast::{self, Receiver},
    time::{interval, interval_at, sleep_until, Instant},
};
use tokio_tungstenite::{
    tungstenite::{
        client::IntoClientRequest,
        http::header::HeaderValue,
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};

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

#[derive(Debug, Deserialize)]
struct ConnectResponse {
    url: String,
}

#[derive(Debug, Deserialize)]
struct GatewayPayload {
    op: u8,
    d: Option<JsonValue>,
    s: Option<u32>,
    t: Option<String>,
}

struct DiscordWebsocket {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    shutdown_rx: Receiver<ShutdownSignal>,
}

fn open_websocket(
    token: String,
    shutdown_rx: Receiver<ShutdownSignal>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ws = DiscordWebsocket::new(&token, shutdown_rx).await;
        let heartbeat_interval = ws.receive_hello().await;
        ws.keep_alive(heartbeat_interval).await;
    })
}

impl DiscordWebsocket {
    async fn new(token: &str, shutdown_rx: Receiver<ShutdownSignal>) -> DiscordWebsocket {
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
        let (ws, _) = res.unwrap();
        DiscordWebsocket { ws, shutdown_rx }
    }

    fn close(&mut self, code: Option<(CloseCode, &str)>) {
        print!("Closing socket... ");
        let code = code.map(|(code, reason)| CloseFrame {
            code,
            reason: reason.into(),
        });
        self.ws.close(code);
        println!("Socket closed");
    }

    async fn receive_hello(&mut self) -> Duration {
        let response = self.ws.next().await.unwrap().unwrap();
        if let Message::Text(response) = response {
            let response: GatewayPayload = serde_json::from_str(&response).unwrap();
            assert_eq!(response.op, HELLO);
            let heartbeat_interval = response.d.unwrap().as_object().unwrap()["heartbeat_interval"]
                .as_u64()
                .unwrap();
            Duration::from_millis(heartbeat_interval)
        } else {
            panic!();
        }
    }

    async fn keep_alive(mut self, heartbeat_interval: Duration) {
        let heartbeat_start_time = Instant::now() + heartbeat_interval.mul_f32(fastrand::f32());
        let mut heartbeat_interval = interval_at(heartbeat_start_time, heartbeat_interval);
        println!("running");
        let mut waiting_for_heartbeat_ack = false;
        loop {
            tokio::select!(
                response = self.ws.next() => {
                    if let Some(Ok(msg)) = response {
                        let response: GatewayPayload = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
                        println!("Message received: {:?}", response);
                    } else {
                        println!("Error message received: {:?}", response);
                    }
                }
                _ = heartbeat_interval.tick() => {
                    if waiting_for_heartbeat_ack {
                        self.close(Some((CloseCode::Protocol, "Connection zombied")));
                        todo!("attempt to resume");
                    } else {
                        self.send_heartbeat().await;
                        waiting_for_heartbeat_ack = true;
                    }
                }
                signal = self.shutdown_rx.recv() => {
                    match signal {
                        Ok(ShutdownSignal) => {
                            println!("Shutdown signal received");
                            break;
                        },
                        Err(err) => {
                            println!("Shutting down due to error in shutdown signal: {}", err);
                            break;
                        },
                    }
                }

            );
        }
        self.close(None);
    }

    async fn send_heartbeat(&mut self) {
        let payload = json!({
            "op": HEARTBEAT,
            "d": null,
        });
        let message = Message::Text(payload.to_string());
        self.ws.send(message).await;
        println!("heartbeat sent");
    }
}

#[derive(Debug, Clone, Copy)]
struct ShutdownSignal;

#[cfg(test)]
#[tokio::test]
async fn test_gateway() {
    let bot_token = dotenv::var("BOT_TOKEN").unwrap();
    let (shutdown_tx, shutdown_rx) = broadcast::channel::<ShutdownSignal>(1);
    let a = open_websocket(bot_token, shutdown_tx.subscribe());
    tokio::signal::ctrl_c().await;
    shutdown_tx.send(ShutdownSignal);
    println!("shutdown signal sent");
    a.await;
}

#[cfg(test)]
mod test {
    #[macro_export]
    macro_rules! this_module {
        () => {{
            println!("hello from module test");
            module_path!()
        }};
    }

    #[test]
    fn test_macro() {
        use crate::this_module;

        println!("{}", this_module!());
    }
}
