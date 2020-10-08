pub mod session;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![];
    routes.append(&mut session::routes());
    routes
}
