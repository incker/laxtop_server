#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[cfg(test)] mod tests;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: i8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>/<age>", rank = 2)]
fn hi(name: String, age: &RawStr) -> String {
    format!("Hi {}! Your age ({}) is kind of funky.", name, age)
}

fn main() {
    rocket::ignite().mount("/", routes![hi, hello]).launch();
}
