use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::guard::{DbConn, SupplierId};
use crate::model::{RespApiKey, Session, Supplier};

#[get("/get-check-telegram-bot-added")]
fn get_check_telegram_bot_added(supplier_id: SupplierId, db_conn: DbConn) -> Json<RespApiKey> {
    let has_telegram_added = Supplier::get_chat_id(supplier_id.into(), db_conn.deref()).is_some();
    // logged_in here means that telegram already added
    Json(RespApiKey {
        key: "".into(),
        logged_in: has_telegram_added,
    })
}

#[get("/get-generate-telegram-token")]
fn get_generate_telegram_token(supplier_id: SupplierId, db_conn: DbConn) -> Json<RespApiKey> {
    let has_telegram_added = Supplier::get_chat_id(supplier_id.0, db_conn.deref()).is_some();

    let key = if has_telegram_added {
        "".to_string()
    } else {
        Session::new_session_telegram(supplier_id.into(), db_conn.deref()).hash
    };

    Json(RespApiKey {
        key,
        logged_in: has_telegram_added,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_check_telegram_bot_added, get_generate_telegram_token,]
}
