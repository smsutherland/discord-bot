struct PartialServerIntegration; // TODO: Create PartialServerIntegration struct

type Snowflake = u64; // TODO: Create Snowflake struct
type ISO8601Timestamp = u64; // TODO: Research ISO8601 Timestamp
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

enum VisibilityType {
    None,
    Everyone,
}
