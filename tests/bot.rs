use discord_bot::*;

#[test]
fn create_bot() {
    let bot_token = dotenv::var("BOT_TOKEN").unwrap();
    let bot = client::Client::new(&bot_token);
    let guilds = bot.fetch_guilds().call();
    println!("{:?}", guilds);
}
