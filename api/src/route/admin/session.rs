use rocket_contrib::json::Json;

use crate::guard::Logout;
use crate::model::RespApiKey;

#[post("/logout")]
pub fn logout(logout: Logout) -> Json<RespApiKey> {
    logout.json()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![logout,]
}
