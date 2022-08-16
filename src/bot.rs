use serde::{self, Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Chat {
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
    pub async fn get_updates(&self) -> Vec<Update> {
        let api = format!("{}/getUpdates", self.telegram_endpoint);
        let body = reqwest::get(api).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
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

