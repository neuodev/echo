use serde::{self, Deserialize, Serialize};
use serde_json::{Value};
use tokio::{task::JoinHandle, fs::{File, self, OpenOptions}, io::AsyncWriteExt};
use std::{env, thread, time::Duration, path::Path};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    id: i64,
    first_name: String,
    last_name: String,
    username: String,
    #[serde(rename = "type")]
    chat_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message_id: u32,
    chat: Chat,
    date: i64,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    update_id: i64,
    message: Message,
}

pub struct Bot {
    telegram_endpoint: String,
    exchange_rates_endpoint: String,
}

impl Bot {
    pub fn new() -> Self {
        let bot_auth_token = env::var("BOT_AUTH_TOKEN").expect("BOT_AUTH_TOKEN is missing");
        let exchange_rate_token = env::var("EXCHAINGE_RATES_API_TOKEN").expect("EXCHAINGE_RATE_API_TOKEN is missing");
        Bot {
            telegram_endpoint: format!("https://api.telegram.org/bot{}", bot_auth_token),
            exchange_rates_endpoint: format!("https://openexchangerates.org/api/latest.json?app_id={}", exchange_rate_token)
        }
    }

    /// Offical docs for this endpoint https://core.telegram.org/bots/api#getupdates
    pub async fn get_updates() -> Vec<Update> {
        let bot_auth_token = env::var("BOT_AUTH_TOKEN").expect("BOT_AUTH_TOKEN is missing");
        let telegram_endpoint = format!("https://api.telegram.org/bot{}", bot_auth_token);
        let api = format!("{}/getUpdates", telegram_endpoint);
        let body = reqwest::get(api).await.unwrap().text().await.unwrap();

        let body: Value = serde_json::from_str::<Value>(&body).unwrap();
        let result = body.get("result").unwrap();
        let updates: Vec<Update> = serde_json::from_value(result.clone()).unwrap();

        updates
    }

    pub async fn send_message(&self, msg: &MessageBody) {
        let api = format!("{}/sendMessage", self.telegram_endpoint);
        let client = reqwest::Client::new();
        let _ = client.post(api).json(&msg).send().await.unwrap();
    }

    pub async fn send_exchange_rates_msg(&self, chat_id: &str) {
        let rates = self.get_exchange_rates().await;
        let msg = MessageBody::new_exchange_rates_msg(chat_id, &rates);
        self.send_message(&msg).await;
    }

    pub async fn get_exchange_rates(&self) -> ExchangeRates {
        let res = reqwest::get(&self.exchange_rates_endpoint).await.unwrap();
        let body = res.text().await.unwrap();
        let rates: ExchangeRates = serde_json::from_str(&body).unwrap();
        
        rates
    }


    pub fn poll_updates() -> JoinHandle<()> {
        tokio::spawn(async {
            let mut iter = 1;
            loop {
                println!("> [update]: {iter}");
                let mut all_chats = Bot::load_bot_data().await;
                Bot::get_updates().await.into_iter().for_each(|update| {
                    all_chats.push(update.message.chat);
                });

                // Remove duplicates
                let mut hs = HashMap::new();

                for chat in all_chats {
                    hs.insert(chat.id, chat);
                }

                let chats = hs.into_iter().map(|(_, value)| value).collect();
                Bot::update_bot_data(chats).await;
                thread::sleep(Duration::from_secs(6)); // Every 10 mins

                iter+=1
            }
        })
    }


    pub async fn load_bot_data() -> Vec<Chat> {
        let path = Path::new("data.json");
        if !path.exists() {
            File::create(path).await.expect("Unable to create data.json");
        }

        let content = fs::read_to_string(path).await.expect("Failed to read the file");

        if content.is_empty() {
            return vec![]
        }

        let chats: Vec<Chat> = serde_json::from_str(&content).unwrap();

        chats
    }


    pub async fn update_bot_data(chats: Vec<Chat>) {
        let path = Path::new("data.json");
        let mut f = OpenOptions::new().write(true).open(path).await.unwrap();
        let json_str = serde_json::to_string(&chats).unwrap();
        f.write_all(json_str.as_bytes()).await.unwrap();
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct MessageBody {
    chat_id: String,
    text: String,
    parse_mode: &'static str,
}

impl MessageBody {
    pub fn new(chat_id: &str, text: &str) -> Self {
        MessageBody {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: PraseMode::MarkdownV2.as_text(),
        }
    }

    pub fn new_exchange_rates_msg(chat_id: &str, rates: &ExchangeRates) -> MessageBody {
        let text = format!(r#"
        *Prices updates*
        1 USD = {} EGP
        1 USD = {} SAR
        "#, rates.rates.egp, rates.rates.sar);
        
        MessageBody::new(chat_id, &text)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum PraseMode {
    MarkdownV2,
    HTML,
    Markdown,
}

impl PraseMode {
    pub fn as_text(&self) -> &'static str {
        match self {
            PraseMode::HTML => "HTM",
            PraseMode::Markdown => "Markdown",
            PraseMode::MarkdownV2 => "MarkdownV2",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRates {
    timestamp: i64,
    base: String,
    rates: Rates
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rates {
    #[serde(rename = "EGP")]
    egp: f64,
    #[serde(rename = "SAR")]
    sar: f64,
}

