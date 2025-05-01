use rocket::Route;

pub mod user_routes;

pub fn user_routes() -> Vec<Route> {
    user_routes::routes()
}
