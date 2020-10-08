use base::valid::{FormErrors, ValidateFormatter};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, sql_query};
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_types::{Tinyint, Varchar};
use guards::DbConn;
use guards::SupplierId;
use models;
use models::RespProduct;
use regex::Regex;
use rocket::http::Status;
use rocket::Response;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::ops::Deref;
use validator::Validate;

#[get("/test")]
fn test() -> Result<status::Custom<Response<'static>>, String> {
    let _codes = [93, 355, 21, 684, 376, 244, 1264, 1268, 374, 54, 61, 43, 994, 1242, 973, 880, 1246, 375, 32, 501, 229, 1441, 591, 387, 267, 55, 1284, 673, 359, 226, 257, 7, 855, 237, 1, 238, 1345, 236, 235, 56, 86, 672, 672, 57, 1670, 269, 242, 682, 506, 385, 53, 357, 420, 45, 246, 253, 1767, 1809, 62, 593, 20, 503, 240, 372, 251, 298, 500, 679, 358, 33, 590, 594, 689, 241, 220, 995, 49, 233, 350, 30, 299, 1473, 1671, 671, 502, 224, 245, 592, 509, 504, 852, 36, 354, 91, 62, 98, 964, 353, 972, 39, 225, 1876, 81, 962, 7, 254, 686, 996, 965, 856, 371, 961, 266, 231, 21, 41, 370, 352, 853, 389, 261, 265, 60, 960, 223, 356, 692, 596, 222, 230, 1706, 1905, 52, 691, 377, 976, 1664, 212, 258, 95, 264, 674, 977, 31, 599, 687, 64, 505, 227, 234, 683, 672, 850, 967, 670, 47, 968, 92, 507, 675, 595, 51, 63, 48, 351, 1787, 974, 378, 262, 40, 7, 250, 247, 508, 39, 239, 966, 221, 248, 232, 65, 421, 386, 677, 252, 27, 82, 969, 34, 94, 1869, 1758, 1784, 249, 597, 47, 268, 46, 41, 963, 992, 886, 255, 66, 228, 690, 676, 1868, 21, 90, 993, 1649, 688, 1340, 256, 380, 971, 44, 598, 1, 998, 678, 39, 58, 84, 681, 21, 685, 381, 243, 260, 263];
    Ok(status::Custom(Status::from_code(401).unwrap(), crate::index_html().unwrap()))
}


#[get("/test2")]
fn test2(db_conn: DbConn) -> String {
    // let res = models::CountryPhoneCode::new_from_phone(db_conn.deref(), &380951970331).unwrap();
    let res = models::CountryPhoneCode::new_from_phone(db_conn.deref(), &770951970331).unwrap();
    println!("{:?}", res);
    "{\"test\":true}".to_string()
}


#[catch(404)]
fn not_found(req: &Request) -> Result<status::Custom<Response<'static>>, String> {
    // TODO: files (like .jpg) are also 404
    let uri = req.uri().to_string();
    if &uri == "/api" || uri.len() > 4 && &uri[0..5] == "/api/" {
        Err(format!("Wrong API uri: {}", uri))
    } else {
        Ok(status::Custom(Status::Ok, index_html().unwrap()))
    }
}


#[post("/bot/telegram/test-dev", format = "application/json", data = "<update>")]
pub fn telegram(update: Json<tgbot::types::Update>, db_conn: RustyDbConn) -> String {
    println!("{:?}", update);
    "".into()
}


#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available. (Is the database up?)"
}


#[get("/test3")]
fn test3(db_conn: DbConn) -> String {
    let users: Vec<DBImage> = sql_query("SELECT CONCAT('/', `dir`, '/', `dir2`, '/', `hash`, '.', `ext`) AS src FROM `image` WHERE 1;")
        .load(&db_conn.0).unwrap();

    println!("{:?}", users);

    "hi".to_string()
}


#[get("/test4")]
fn test4(db_conn: DbConn) -> String {
    use schema::supplier_session;
    use schema::supplier_session::dsl::*;

    let res = supplier_session::table
        .inner_join(supplier_product_key::table)
        .limit(5)
        // .order(news::date.desc())
        .load::<(models::SupplierSession, models::SupplierProductKey)>(&db_conn.0) // To this point we get the result as a tuple.
        .expect("Error loading news"); // Another panic waiting to happen!

    format!("{:?}", res)
}


pub fn routes() -> Vec<rocket::Route> {
    routes![
        test,
        test3,
    ]
}


/*
fn get_rect(lat: &f32, lng: &f32) -> Rect<f32> {
    let (lat_min, lat_max) = {
        let value = (lat * 100f32).floor();
        (value / 100f32, (value + 1.).round() / 100f32)
    };
    let (lng_min, lng_max) = {
        let value = (lng * 100f32).floor();
        (value / 100f32, (value + 1.).round() / 100f32)
    };
    assert!(lng_min < lng_max);
    assert!(lat_min < lat_max);

    Rect::new(
        Coordinate { x: lng_min, y: lat_min },
        Coordinate { x: lng_max, y: lat_max },
    )
}
*/
