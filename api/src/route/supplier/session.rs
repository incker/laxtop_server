use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{ApiKey, DbConn, Logout, SupplierId};
use crate::model::{RespApiKey, SignInFields, TelegramOauth};

#[post("/login", format = "application/json", data = "<login_input>")]
fn login(login_input: Json<SignInFields>, db_conn: DbConn) -> Json<RespApiKey> {
    match login_input.supplier_login(db_conn.deref()) {
        Ok((_, resp_api_key)) => Json(resp_api_key),
        Err(_) => Json(RespApiKey::default()),
    }
}

#[post("/telegram-login", format = "application/json", data = "<oauth_data>")]
fn telegram_login(
    oauth_data: Json<TelegramOauth>,
    db_conn: DbConn,
) -> Result<Json<RespApiKey>, Json<RespErrors>> {
    let telegram_user = oauth_data
        .into_inner()
        .check_telegram_authorization()
        .map_err(|_| Json(RespErrors::new_error(("login".into(), "invalid".into()))))?;

    telegram_user.insert_or_update(db_conn.deref());

    telegram_user
        .login_as_supplier(db_conn.deref())
        .map(|(_, resp_api_key)| Json(resp_api_key))
        .ok_or_else(|| {
            Json(RespErrors::new_error((
                "login".into(),
                "Supplier does not exist".into(),
            )))
        })
}

#[get("/check-login-status")]
pub fn check_login_status(api_key: ApiKey, _supplier_id: SupplierId) -> Json<RespApiKey> {
    Json(RespApiKey::new_logged_in(api_key.0.to_string()))
}

#[post("/logout")]
pub fn logout(logout: Logout) -> Json<RespApiKey> {
    logout.json()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, telegram_login, check_login_status, logout]
}
