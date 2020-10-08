pub mod data;
pub mod invoice;
pub mod session;
pub mod spot;
pub mod supplier;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![];
    routes.append(&mut session::routes());
    routes.append(&mut data::routes());
    routes.append(&mut invoice::routes());
    routes.append(&mut supplier::routes());
    routes.append(&mut spot::routes());
    routes
}
