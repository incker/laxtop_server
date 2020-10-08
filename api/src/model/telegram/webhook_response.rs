use serde::Serialize;
use tgbot::methods::SendMessage;

#[derive(Serialize)]
pub struct WebhookResponse<M> {
    pub method: String,
    #[serde(flatten)]
    pub data: M,
}

impl From<SendMessage> for WebhookResponse<SendMessage> {
    fn from(send_message: SendMessage) -> Self {
        WebhookResponse {
            method: "sendMessage".to_string(),
            data: send_message,
        }
    }
}
