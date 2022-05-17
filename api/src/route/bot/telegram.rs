use tgbot::{
    Api,
    methods::{SendMessage, SetWebhook},
    types::ParseMode,
};
use tokio::runtime::Runtime;

pub fn send_msg(api: Api, chat_id: i64, msg: &str) -> Result<(), String> {
    let method = SendMessage::new(chat_id, msg).parse_mode(ParseMode::Html);

    // !!! looks like in future rocket 0.5 no need to create runtime, just call block_on function from use futures::executor::block_on;
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(async {
            api.execute(method)
                .await
                .map(|_| ())
                .map_err(|e| format!("{:?}", e))
        })
}

pub fn set_webhook() -> Result<(), String> {
    let api = get_telegram_api();

    let webhook = {
        let mut url = dotenv::var("DOMAIN_NAME").expect("DOMAIN_NAME is not set");
        url.push_str("/api/bot/keg47q3t3iaf8kn8h6n2uoz201ou0oo4ap0e4w1d");
        SetWebhook::new(url)
    };

    // !!! looks like in future rocket 0.5 no need to create runtime, just call block_on function from use futures::executor::block_on;
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(async {
            api.execute(webhook)
                .await
                .map(|_res| {})
                .map_err(|err| format!("Webhook was not set: {:?}", err))
        })
}

pub fn get_telegram_api() -> Api {
    let token = dotenv::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    Api::new(token).expect("Failed to create API")
}
