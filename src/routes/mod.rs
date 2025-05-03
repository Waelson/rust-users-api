use rocket::Route;

pub mod cors_options;
pub mod user_routes;

pub fn user_routes() -> Vec<Route> {
    user_routes::routes()
}
