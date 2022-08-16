mod bot;

use bot::Bot;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let bot = Bot::new();

    // bot.get_updates().await;
    // bot.send_message().await;
    // bot.get_exchange_rates().await;
    Bot::poll_updates().await.unwrap();
    Ok(())
}

// Todo: 1. Error handling 
// Todo: 2. save user data into json file
// Todo: 3. Extened the logic to include gold as well