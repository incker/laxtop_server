pub mod about;
pub mod catalog;
pub mod coverage;
pub mod promo;
pub mod session;
pub mod telegram;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![];
    routes.append(&mut about::routes());
    routes.append(&mut catalog::routes());
    routes.append(&mut coverage::routes());
    routes.append(&mut session::routes());
    routes.append(&mut promo::routes());
    routes.append(&mut telegram::routes());
    routes
}
