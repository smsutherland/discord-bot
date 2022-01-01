use crate::request::Request;
use json::JsonValue;
use websocket::{ClientBuilder, Message};

struct DiscordGateway;

impl DiscordGateway {
    fn connect(token: &str) {
        let gateway_request = Request::get("/gateway/bot")
            .authorize(token)
            .with_param("v", "9")
            .with_param("encoding", "json");

        let response =
            json::parse(&gateway_request.call().unwrap().into_string().unwrap()).unwrap();
        println!("{}", response.pretty(4));

        let url = response["url"].as_str().unwrap();
        println!("connecting to {}", url);

        let mut auth_header = websocket::header::Headers::new();
        auth_header.set(websocket::header::Authorization(format!("Bot {}", token)));

        let mut client = ClientBuilder::new(&format!("{}?v=9&encoding=json", url))
            .unwrap()
            .custom_headers(&auth_header)
            .connect_secure(None)
            .unwrap();

        let response = client.recv_message().unwrap();
        if let websocket::OwnedMessage::Text(response) = response{
            let response = json::parse(&response).unwrap();
            println!("{}", response.pretty(4));
        }

        client.shutdown();
    }
}

struct GatewayPayload {
    op: u32,
    d: Option<JsonValue>,
    s: Option<u32>,
    t: Option<String>,
}

#[cfg(test)]
mod gateway_test {
    use super::*;
    #[test]
    fn gateway() {
        let bot_token = dotenv::var("BOT_TOKEN").unwrap();
        DiscordGateway::connect(&bot_token);
    }
}
