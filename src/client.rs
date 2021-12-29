use json;
use ureq;

pub struct Client {
    id: u64,
    username: String,
    avatar: String,
    discriminator: String,
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

        Ok(Client {
            id: response["id"].as_str().unwrap().parse().unwrap(),
            username: String::from(response["username"].as_str().unwrap()),
            avatar: String::from(response["avatar"].as_str().unwrap()),
            discriminator: response["discriminator"]
                .as_str()
                .map(String::from)
                .unwrap(),
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

    /// Get a reference to the client's id.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get a reference to the client's username.
    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    /// Get a reference to the client's avatar.
    pub fn avatar(&self) -> &str {
        self.avatar.as_ref()
    }

    /// Get a reference to the client's discriminator.
    pub fn discriminator(&self) -> &str {
        self.discriminator.as_ref()
    }

    /// Get a reference to the client's public flags.
    pub fn public_flags(&self) -> u64 {
        self.public_flags
    }

    /// Get a reference to the client's flags.
    pub fn flags(&self) -> u64 {
        self.flags
    }

    /// Get a reference to the client's bot.
    pub fn bot(&self) -> bool {
        self.bot
    }

    /// Get a reference to the client's banner.
    pub fn banner(&self) -> Option<&String> {
        self.banner.as_ref()
    }

    /// Get a reference to the client's banner color.
    pub fn banner_color(&self) -> Option<&String> {
        self.banner_color.as_ref()
    }

    /// Get a reference to the client's accent color.
    pub fn accent_color(&self) -> Option<&String> {
        self.accent_color.as_ref()
    }

    /// Get a reference to the client's bio.
    pub fn bio(&self) -> &str {
        self.bio.as_ref()
    }

    /// Get a reference to the client's locale.
    pub fn locale(&self) -> &str {
        self.locale.as_ref()
    }

    /// Get a reference to the client's mfa enabled.
    pub fn mfa_enabled(&self) -> bool {
        self.mfa_enabled
    }

    /// Get a reference to the client's email.
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    /// Get a reference to the client's verified.
    pub fn verified(&self) -> bool {
        self.verified
    }
}
