use discord_bot::*;

#[tokio::test]
async fn create_bot() {
    let bot_token = dotenv::var("BOT_TOKEN").unwrap();
    let bot = client::Client::new(&bot_token).await;
    let guilds = bot.fetch_guilds().call().await;
    println!("{:?}", guilds);
}
