use crate::user::User;

pub struct Client {
    user: User,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            user: User::from_bot_token(token),
        }
    }
}
