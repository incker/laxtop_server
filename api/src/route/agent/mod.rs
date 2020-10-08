pub mod session;
pub mod spot;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![];
    routes.append(&mut session::routes());
    routes.append(&mut spot::routes());
    routes
}
