struct Role; // TODO: Create Role struct
struct Emoji; // TODO: Create Emoji struct
struct PartialVoiceState; // TODO: Create PartialVoiceState struct
struct Channel; // TODO: Create Channel struct
struct PartialPresenceUpdate; // TODO: Create PartialPresenceUpdate struct
struct StageInstance; // TODO: Create StageInstance struct
struct Sticker; // TODO: Create Sticker struct
struct GuildScheduledEvent; // TODO: Create GuildScheduledEvent struct
struct User; // TODO: Create User struct

type Snowflake = u64; // TODO: Create Snowflake struct
type ISO8601Timestamp = u64; // TODO: Research ISO8601 Timestamp
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

struct GuildWidget {
    enabled: bool,
    channel_id: Option<Snowflake>,
}

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

struct IntegrationAccount {
    id: String,
    name: String,
}

struct IntegrationApplication {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    description: String,
    summary: String,
    bot: Option<User>,
}

struct Ban {
    reason: Option<String>,
    user: User,
}

struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>,
}

struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: String,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
}

enum DefaultMessageNotificationLevel {
    AllMessages,
    OnlyMentions,
}

enum ExplicitContentFilterLevel {
    Disabled,
    MembersWithoutRoles,
    AllMembers,
}

enum MFALevel {
    None,
    Elevated,
}

enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

enum GuildNSFWLevel {
    Default,
    Explicit,
    Safe,
    AgeRestricted,
}

enum PremiumTier {
    None,
    Tier1,
    Tier2,
    Tier3,
}

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

enum IntegrationExpireBehavior {
    RemoveRole,
    Kick,
}
