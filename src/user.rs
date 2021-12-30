struct PartialServerIntegration; // TODO: Create PartialServerIntegration struct

type Snowflake = u64; // TODO: Create Snowflake struct
pub struct User {
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

impl User {
    pub fn from_bot_token(token: &str) -> User {
        let request_url = "https://discord.com/api/v9/users/@me";

        let response = json::parse(
            &ureq::get(request_url)
                .set("Authorization", &format!("Bot {}", token))
                .call()
                .unwrap()
                .into_string()
                .unwrap(),
        )
        .unwrap();

        User {
            id: response["id"].as_str().unwrap().parse().unwrap(),
            username: String::from(response["username"].as_str().unwrap()),
            discriminator: response["discriminator"]
                .as_str()
                .map(String::from)
                .unwrap(),
            avatar: response["avatar"].as_str().map(String::from),
            bot: response["bot"].as_bool(),
            system: response["system"].as_bool(),
            mfa_enabled: Some(response["mfa_enabled"].as_bool().unwrap()),
            banner: response["banner"].as_str().map(String::from),
            accent_color: response["accent_color"].as_u64(),
            locale: Some(String::from(response["locale"].as_str().unwrap())),
            verified: Some(response["verified"].as_bool().unwrap()),
            email: response["email"].as_str().map(String::from),
            flags: Some(response["flags"].as_u64().unwrap()),
            premium_type: response["premium_type"].as_u8().map(PremiumType::from),
            public_flags: Some(response["public_flags"].as_u64().unwrap()),
        }
    }
}

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

enum VisibilityType {
    None,
    Everyone,
}
