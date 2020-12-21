use std::convert::Into;
use std::ops::Deref;

use diesel::MysqlConnection;
use rocket_contrib::json::Json;
use tgbot::methods::SendMessage;
use tgbot::types::UpdateKind;

use crate::guard::{self, DbConn};
use crate::model::{Session, Supplier, WebhookResponse};

pub mod telegram;

#[post(
    "/keg47q3t3iaf8kn8h6n2uoz201ou0oo4ap0e4w1d",
    format = "application/json",
    data = "<update>"
)]
pub fn telegram_webhook(
    update: Json<tgbot::types::Update>,
    db_conn: DbConn,
) -> Result<Json<WebhookResponse<SendMessage>>, String> {
    let Json(up) = update;

    if let UpdateKind::Message(message) = up.kind {
        if let Some(text) = message.get_text() {
            let chat_id = message.get_chat_id();
            let user_id = message.get_user().ok_or("")?.id;
            let response = handle_telegram_text(&text.data, chat_id, user_id, db_conn.deref());
            if let Some(resp) = response {
                return Ok(Json(WebhookResponse::from(SendMessage::new(chat_id, resp))));
            }
        }
    }

    Err("".into())
}

fn handle_telegram_text(
    text: &str,
    chat_id: i64,
    telegram_user_id: i64,
    conn: &MysqlConnection,
) -> Option<String> {
    let supplier = Supplier::get_by_chat_id(chat_id, conn);

    let first_char = text.chars().next();
    if Some('/') == first_char {
        let parts: Vec<&str> = text.splitn(2, ' ').collect();
        let (command, data) = if parts.len() == 2 {
            (parts[0], parts[1].trim())
        } else {
            (parts[0], "")
        };

        // println!("command: {}", command);
        // println!("data: {}", data);

        if command == "/start" {
            if supplier.is_some() {
                return Some("Вы уже подписаны на уведомления".to_string());
            } else if !data.is_empty() {
                let found_supplier: Option<guard::SupplierId> =
                    Session::get_telegram_session(data, conn);
                if let Some(guard::SupplierId(supplier_id)) = found_supplier {
                    return if Supplier::get_chat_id(supplier_id, conn).is_none() {
                        Supplier::set_chat_id(supplier_id, chat_id, telegram_user_id, conn);
                        Some("Уведомления настроены!".to_string())
                    } else {
                        // exist supplier, but already subscribed // ideal: write to file
                        Some("Какая-то ошибка которой быть не должно".to_string())
                    };
                } else {
                    return Some("Проверочный код не валидный".to_string());
                }
            }
        } else {
            return Some("Извините, я не знаю такой команды".to_string());
        };
    }

    // ideal: to unknown person write something important (about us)
    // ideal: write in db telegram username

    None
}

pub fn routes() -> Vec<rocket::Route> {
    let routes = routes![telegram_webhook,];
    routes
}
