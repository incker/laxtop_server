#![feature(decl_macro)]
#![allow(unused)]

extern crate base64;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate futures;
extern crate geo;
extern crate geo_types;
extern crate image;
extern crate rand;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
extern crate tgbot;
extern crate tokio;
extern crate validator;
#[macro_use]
extern crate validator_derive;

use rocket::fairing::AdHoc;
use rocket::request::Request;
use rocket::Rocket;

use crate::guard::DbConn;
use crate::model::TelegramOauth;

mod base;
mod guard;
mod model;
mod route;
mod schema;
mod telegram;

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    // This macro from `diesel_migrations` defines an `embedded_migrations` module
    // containing a function named `run`. This allows the example to be run and
    // tested without any outside setup of the database.
    embed_migrations!();

    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            panic!("Failed to run database migrations: {:?}", e);
            // todo: log::error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn main() {
    /*
    // env validation
    let _ = dotenv::var("DOMAIN_NAME").expect("DOMAIN_NAME is not set");
    let _ = dotenv::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    let _ = dotenv::var("NODE_PATH").expect("TELEGRAM_TOKEN is not set");

    if cfg!(build = "release") {
        std::thread::spawn(|| {
            route::bot::telegram::set_webhook().unwrap();
        });
    }
    */

    let rocket = get_rocket();
    let e = rocket.launch();
    println!("Rocket launch error: {}", e);
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[get("/test")]
fn test() -> String {
    let telegram_oauth = TelegramOauth {
        hash: "5882e9008eb9d8c09ce3afcc881368a291fbf884e160754091d76233cdca4e15".into(),
        id: 482231043,
        username: Some("incker".into()),
        first_name: Some("Stanislav".into()),
        last_name: Some("Sagan".into()),
        photo_url: Some(
            "https://t.me/i/userpic/320/k-TvkyhDY3hD5gp92viU35gr9mi4hlURE8-9AF2-duk.jpg".into(),
        ),
        auth_date: 1588977704,
    };

    // bot_token: String, expired_seconds: u64
    let res = telegram_oauth.check_telegram_authorization();

    println!("{:?}", res);

    "Laxtop Works!".into()
}

#[catch(500)]
fn internal_error(req: &Request) -> &'static str {
    // todo catcher not working(
    // todo in plans to write error in file
    let _uri = req.uri();
    println!("catcher works: '{}'", req.uri());
    "Whoops! Looks like we messed up."
}

fn get_rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/api", routes![test])
        .mount("/api/admin", route::admin::routes())
        .mount("/api/agent", route::agent::routes())
        .mount("/api/bot", route::bot::routes())
        .mount("/api/supplier", route::supplier::routes())
        .mount("/api/user", route::user::routes())
        .register(catchers![not_found, internal_error])
}

// delete #![allow(unused)]
// cargo fix --allow-no-vcs
// ngrok http 3000

// cd C:\dev\prj\laxtop_server
// docker-compose exec api bash

// todo: log to database (easy)
// todo: admin page?

// important:
// todo: test that images saved
// todo: is outdated image path: default_test_spot_image
// todo: default image path id is 0 or 1 ? (modify app sql)
