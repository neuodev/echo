mod bot;

use bot::Bot;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let updates_hanlder = Bot::poll_updates();
    Bot::bradcast_updates().await;
    updates_hanlder.await.unwrap();
    Ok(())
}

// Todo: 1. Error handling
// Todo: 2. save user data into json file
// Todo: 3. Extened the logic to include gold as well
