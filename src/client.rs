use json;
use ureq;

pub struct Client {
    id: u64,
    username: String,
    avatar: String,
    discriminator: u16,
    public_flags: u64,
    flags: u64,
    bot: bool,
    banner: Option<String>,
    banner_color: Option<String>,
    accent_color: Option<String>,
    bio: String,
    locale: String,
    mfa_enabled: bool,
    email: Option<String>,
    verified: bool,
}

impl Client {
    pub fn new(token: &str) -> Result<Client, ureq::Error> {
        let request_url = "https://discord.com/api/v9/users/@me";

        let response = json::parse(
            &ureq::get(request_url)
                .set("Authorization", &format!("Bot {}", token))
                .call()?
                .into_string()?,
        )
        .unwrap();

        println!("{}", response);

        Ok(Client {
            id: response["id"].as_str().unwrap().parse().unwrap(),
            username: String::from(response["username"].as_str().unwrap()),
            avatar: String::from(response["avatar"].as_str().unwrap()),
            discriminator: response["discriminator"].as_str().unwrap().parse().unwrap(),
            public_flags: response["public_flags"].as_u64().unwrap(),
            flags: response["flags"].as_u64().unwrap(),
            bot: response["bot"].as_bool().unwrap(),
            banner: response["banner"].as_str().map(String::from),
            banner_color: response["banner_color"].as_str().map(String::from),
            accent_color: response["accent_color"].as_str().map(String::from),
            bio: String::from(response["bio"].as_str().unwrap()),
            locale: String::from(response["locale"].as_str().unwrap()),
            mfa_enabled: response["mfa_enabled"].as_bool().unwrap(),
            email: response["email"].as_str().map(String::from),
            verified: response["verified"].as_bool().unwrap(),
        })
    }
}
