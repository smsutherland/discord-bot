use crate::user::User;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Role; // TODO: Create Role struct
#[derive(Debug, Deserialize)]
struct Emoji; // TODO: Create Emoji struct
#[derive(Debug, Deserialize)]
struct PartialVoiceState; // TODO: Create PartialVoiceState struct
#[derive(Debug, Deserialize)]
struct Channel; // TODO: Create Channel struct
#[derive(Debug, Deserialize)]
struct PartialPresenceUpdate; // TODO: Create PartialPresenceUpdate struct
#[derive(Debug, Deserialize)]
struct StageInstance; // TODO: Create StageInstance struct
#[derive(Debug, Deserialize)]
struct Sticker; // TODO: Create Sticker struct
#[derive(Debug, Deserialize)]
struct GuildScheduledEvent; // TODO: Create GuildScheduledEvent struct

type Snowflake = u64; // TODO: Create Snowflake struct
type ISO8601Timestamp = u64; // TODO: Research ISO8601 Timestamp
#[derive(Debug, Deserialize)]
pub struct Guild {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: Option<bool>,
    owner_id: Snowflake,
    permissions: Option<String>,
    region: Option<String>,
    afk_channel_id: Option<Snowflake>,
    afk_timeout: u64,
    widget_enabled: Option<bool>,
    widget_channel_id: Option<Snowflake>,
    verification_level: VerificationLevel,
    default_message_notifications: DefaultMessageNotificationLevel,
    explecit_content_filter: ExplicitContentFilterLevel,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<GuildFeature>,
    mfa_level: MFALevel,
    application_id: Option<Snowflake>,
    system_channel_id: Option<Snowflake>,
    system_channel_flags: u64,
    rules_channel_id: Option<Snowflake>,
    joined_at: ISO8601Timestamp,
    large: Option<bool>,
    unavailable: Option<bool>,
    member_count: Option<u64>,
    voice_states: Option<Vec<PartialVoiceState>>,
    members: Option<Vec<GuildMember>>,
    channels: Option<Vec<Channel>>,
    threads: Option<Vec<Channel>>,
    presences: Option<Vec<PartialPresenceUpdate>>,
    max_presences: Option<u64>,
    max_members: Option<u64>,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: PremiumTier,
    premium_subscription_count: Option<u64>,
    preferred_locale: String,
    public_updates_channel_id: Option<Snowflake>,
    max_video_channel_users: Option<u64>,
    approximate_member_count: Option<u64>,
    approximate_presence_count: Option<u64>,
    welcome_screen: Option<WelcomeScreen>,
    nsfw_level: GuildNSFWLevel,
    stage_instanes: Option<Vec<StageInstance>>,
    stickers: Option<Vec<Sticker>>,
    guild_scheduled_evens: Option<Vec<GuildScheduledEvent>>,
    premium_progress_bar_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct PartialGuild {
    #[serde(deserialize_with = "string_to_int")]
    id: u64,
    name: String,
    icon: Option<String>,
    owner: bool,
    #[serde(deserialize_with = "string_to_int")]
    permissions: u64,
    features: Vec<GuildFeature>,
}

fn string_to_int<'de, D>(deserializer: D) -> Result<Snowflake, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(D::Error::custom)
}

#[derive(Debug, Deserialize)]
struct GuildPreview {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    emojis: Vec<Emoji>,
    features: Vec<GuildFeature>,
    approximate_member_count: u64,
    approximate_presence_count: u64,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GuildWidget {
    enabled: bool,
    channel_id: Option<Snowflake>,
}

#[derive(Debug, Deserialize)]
struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    avatar: Option<String>,
    roles: Vec<Snowflake>,
    joined_at: ISO8601Timestamp,
    premium_since: Option<ISO8601Timestamp>,
    deaf: bool,
    mute: bool,
    pending: Option<bool>,
    permissions: Option<String>,
    communication_disabled_until: Option<ISO8601Timestamp>,
}

#[derive(Debug, Deserialize)]
struct Integration {
    id: Snowflake,
    name: String,
    r#type: String,
    enabled: bool,
    syncing: Option<bool>,
    role_id: Option<Snowflake>,
    enable_emoticons: Option<bool>,
    expire_behavior: Option<IntegrationExpireBehavior>,
    expire_grace_period: Option<u64>,
    user: Option<User>,
    account: IntegrationAccount,
    synced_at: Option<ISO8601Timestamp>,
    subscriber_count: Option<u64>,
    revoked: Option<bool>,
    application: Option<IntegrationApplication>,
}

#[derive(Debug, Deserialize)]
struct IntegrationAccount {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct IntegrationApplication {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    description: String,
    summary: String,
    bot: Option<User>,
}

#[derive(Debug, Deserialize)]
struct Ban {
    reason: Option<String>,
    user: User,
}

#[derive(Debug, Deserialize)]
struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Debug, Deserialize)]
struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: String,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
}

#[derive(Debug, Deserialize)]
enum DefaultMessageNotificationLevel {
    AllMessages,
    OnlyMentions,
}

#[derive(Debug, Deserialize)]
enum ExplicitContentFilterLevel {
    Disabled,
    MembersWithoutRoles,
    AllMembers,
}

#[derive(Debug, Deserialize)]
enum MFALevel {
    None,
    Elevated,
}

#[derive(Debug, Deserialize)]
enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Deserialize)]
enum GuildNSFWLevel {
    Default,
    Explicit,
    Safe,
    AgeRestricted,
}

#[derive(Debug, Deserialize)]
enum PremiumTier {
    None,
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Debug, Deserialize)]
enum GuildFeature {
    AnimatedIcon,
    Banner,
    Commerce,
    Community,
    Discoverable,
    Featurable,
    InviteSplash,
    MemberVerificationGateEnabled,
    MonetizationEnabled,
    MoreStickers,
    News,
    Partnered,
    PreviewEnabled,
    PrivateThreads,
    RoleIcons,
    SevenDayThreadArchive,
    ThreeDayThreadArchive,
    TicketedEventsEnabled,
    VanityURL,
    Verified,
    VIPRegions,
    WelcomeScreenEnabled,
}

impl GuildFeature {
    fn from_str(str: &str) -> GuildFeature {
        use GuildFeature::*;
        match str {
            "ANIMATED_ICON" => AnimatedIcon,
            "BANNER" => Banner,
            "COMMERCE" => Commerce,
            "COMMUNITY" => Community,
            "DISCOVERABLE" => Discoverable,
            "FEATURABLE" => Featurable,
            "INVITE_SPLASH" => InviteSplash,
            "MEMBER_VERIFICATION_GATE_ENABLED" => MemberVerificationGateEnabled,
            "MONETIZATION_ENABLED" => MonetizationEnabled,
            "MORE_STICKERS" => MoreStickers,
            "NEWS" => News,
            "PARTNERED" => Partnered,
            "PREVIEW_ENABLED" => PreviewEnabled,
            "PRIVATE_THREADS" => PrivateThreads,
            "ROLE_ICONS" => RoleIcons,
            "SEVEN_DAY_THREAD_ARCHIVE" => SevenDayThreadArchive,
            "THREE_DAY_THREAD_ARCHIVE" => ThreeDayThreadArchive,
            "TICKETED_EVENTS_ENABLED" => TicketedEventsEnabled,
            "VANITY_URL" => VanityURL,
            "VERIFIED" => Verified,
            "VIP_REGIONS" => VIPRegions,
            "WELCOME_SCREEN_ENABLED" => WelcomeScreenEnabled,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Deserialize)]
enum IntegrationExpireBehavior {
    RemoveRole,
    Kick,
}
