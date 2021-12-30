use crate::user::User;

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
}
