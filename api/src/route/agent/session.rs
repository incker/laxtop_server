use std::ops::Deref;

use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::guard::{DbConn, Logout};
use crate::model::{RespApiKey, SignInFields};

#[post("/login", format = "application/json", data = "<login_input>")]
fn login(login_input: Json<SignInFields>, db_conn: DbConn) -> Json<RespApiKey> {
    match login_input.agent_login(db_conn.deref()) {
        Ok((_, resp_api_key)) => Json(resp_api_key),
        Err(_) => Json(RespApiKey::default()),
    }
}

#[post("/logout")]
pub fn logout(_logout: Logout) -> Status {
    Status::from_code(401).unwrap()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, logout]
}
