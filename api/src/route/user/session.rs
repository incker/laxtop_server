use std::ops::Deref;

use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, Logout};
use crate::model::{AuthorizedUserData, FirebaseUserInfo, SignInDetails};

#[post(
    "/firebase-login",
    format = "application/json",
    data = "<sign_in_details>"
)]
fn firebase_login(
    sign_in_details: Json<SignInDetails>,
    db_conn: DbConn,
) -> Result<Json<AuthorizedUserData>, Json<RespErrors>> {
    let firebase_user_info: FirebaseUserInfo = sign_in_details
        .into_inner()
        .get_user_info_by_token()
        .map_err(|firebase_error| Json(firebase_error.into_resp_errors()))?;

    Ok(Json(AuthorizedUserData::new(
        firebase_user_info,
        db_conn.deref(),
    )))
}

#[post("/logout")]
pub fn logout(_logout: Logout) -> Status {
    Status::from_code(401).unwrap()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![firebase_login, logout,]
}
