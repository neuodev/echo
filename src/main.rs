mod bot;

use bot::Bot;
use dotenv::dotenv;

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author="Ahmed Ibrahim", version="0.1.0", about, long_about = None)]
pub struct Args {
    /// Time to poll latest updates from telegram ans store it in a local file
    #[clap(short, long, value_parser, default_value = "10m")]
    update_iterval: String,
    /// Time to send the prices for all users 
    #[clap(short, long, value_parser, default_value = "8h")]
    broadcast_interval: String
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let args = Args::parse();
    println!("Bot Configs: {:#?}", args);
    
    let updates_hanlder = Bot::poll_updates(args.update_iterval);
    Bot::broadcast_updates(args.broadcast_interval).await;
    updates_hanlder.await.unwrap();
    Ok(())
}
