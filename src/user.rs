use crate::request::Request;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug)]
struct PartialServerIntegration; // TODO: Create PartialServerIntegration struct

type Snowflake = u64; // TODO: Create Snowflake struct
#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "string_to_int")]
    id: Snowflake,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Option<String>,
    accent_color: Option<u64>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<u64>,
    premium_type: Option<PremiumType>,
    public_flags: Option<u64>,
}

fn string_to_int<'de, D>(deserializer: D) -> Result<Snowflake, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(D::Error::custom)
}

impl User {
    pub async fn from_bot_token(token: &str) -> User {
        let request = Request::get("/users/@me").authorize(token);

        request.call().await.unwrap().json::<User>().await.unwrap()
    }
}

#[derive(Debug)]
struct Connection {
    id: String,
    name: String,
    r#type: String,
    revoked: Option<bool>,
    integrations: Option<Vec<PartialServerIntegration>>,
    verified: bool,
    friend_sync: bool,
    show_activity: bool,
    visibility: VisibilityType,
}

#[derive(Debug, Deserialize)]
enum PremiumType {
    None,
    NitroClassic,
    Nitro,
}

impl PremiumType {
    fn from(num: u8) -> PremiumType {
        match num {
            0 => PremiumType::None,
            1 => PremiumType::NitroClassic,
            2 => PremiumType::Nitro,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum VisibilityType {
    None,
    Everyone,
}
