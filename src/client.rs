use crate::{user::User, guild::PartialGuild, request::{Request, Route, HTTPMethod}};

type Snowflake = u64; // TODO: Create Snowflake struct

#[derive(Debug)]
pub struct Client {
    user: User,
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            user: User::from_bot_token(token),
            token: String::from(token),
        }
    }

    pub fn fetch_guilds(&self) -> FetchGuilds {
        FetchGuilds::default(&self.token)
    }
}

pub struct FetchGuilds<'a> {
    token: &'a str,
    before: Option<Snowflake>,
    after: Option<Snowflake>,
    limit: Option<u32>,
}

impl<'a> FetchGuilds<'a> {
    fn default(token: &'a str) -> Self {
        FetchGuilds {
            token: token,
            before: None,
            after: None,
            limit: None,
        }
    }

    pub fn before(mut self, before: Snowflake) -> Self {
        self.before = Some(before);
        self
    }

    pub fn after(mut self, after: Snowflake) -> Self {
        self.after = Some(after);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn call(self) -> Vec<PartialGuild>{
        if let Some(limit) = self.limit {
            assert!(limit > 0);
            assert!(limit < 200);
        }

        let mut request = Request::new(Route::new(HTTPMethod::Get, "/users/@me/guilds"))
            .authorize(self.token);

        if let Some(before) = self.before{
            request.add_param("limit", &before.to_string());
        }
        if let Some(after) = self.after{
            request.add_param("limit", &after.to_string());
        }
        if let Some(limit) = self.limit{
            request.add_param("limit", &limit.to_string());
        }

        let response = json::parse(&request.call().unwrap().into_string().unwrap()).unwrap();

        response.members().cloned().map(|guild| PartialGuild::from_json_value(guild)).collect()
    }
}
