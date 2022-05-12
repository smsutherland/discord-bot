use crate::guild::PartialGuild;
use crate::request::Request;
use crate::user::User;

type Snowflake = u64; // TODO: Create Snowflake struct

#[derive(Debug)]
pub struct Client {
    user: User,
    token: String,
}

impl Client {
    pub async fn new(token: &str) -> Client {
        Client {
            user: User::from_bot_token(token).await,
            token: String::from(token),
        }
    }

    pub fn fetch_guilds(&self) -> FetchGuilds {
        FetchGuilds::default(&self.token)
    }
}

/// Represents a builder for a [`fetch_guilds`](Client::fetch_guilds) call on a [`Client`] object.
pub struct FetchGuilds<'a> {
    token: &'a str,
    before: Option<Snowflake>,
    after: Option<Snowflake>,
    limit: Option<u32>,
}

impl<'a> FetchGuilds<'a> {
    fn default(token: &'a str) -> Self {
        FetchGuilds {
            token,
            before: None,
            after: None,
            limit: None,
        }
    }

    /// Specifies the optional parameter `before` on a fetch_guilds call.
    /// Get guilds before this guild ID.
    /// If not specified, there is no upper bound.
    /// Must be in the form of a Discord [Snowflake](https://discord.com/developers/docs/reference#snowflakes).
    pub fn before(mut self, before: Snowflake) -> Self {
        self.before = Some(before);
        self
    }

    /// Specifies the optional parameter `after` on a fetch_guilds call.
    /// Get guilds after this guild ID.
    /// If not specified, there is no lower bound.
    /// Must be in the form of a Discord [Snowflake](https://discord.com/developers/docs/reference#snowflakes).
    pub fn after(mut self, after: Snowflake) -> Self {
        self.after = Some(after);
        self
    }

    /// Specifies the optional parameter `limit` on a fetch_guilds call.
    /// The limit must be in the range 1..=200.
    /// If not specified, the default is 200.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Executes the fetch_guilds call.
    /// Blocks until the request to the Discord API returns. TODO: asynchronous calling.
    ///
    /// ## Panics
    /// The function will panic if specified limit is outside the range 1..=200.
    ///
    /// The function will panic if something funky happens when parsing the return from the Discord API.
    ///
    /// TODO: proper error handling of these things.
    pub async fn call(self) -> Vec<PartialGuild> {
        if let Some(limit) = self.limit {
            assert!(limit > 0);
            assert!(limit < 200);
        }

        let mut request = Request::get("/users/@me/guilds").authorize(self.token);

        if let Some(before) = self.before {
            request.add_param("before", &before.to_string());
        }
        if let Some(after) = self.after {
            request.add_param("after", &after.to_string());
        }
        if let Some(limit) = self.limit {
            request.add_param("limit", &limit.to_string());
        }

        let response = request.call().await.unwrap();
        response.json::<Vec<PartialGuild>>().await.unwrap()
    }
}
