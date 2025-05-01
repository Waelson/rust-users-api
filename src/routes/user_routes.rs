use crate::context::AppContext;
use crate::models::user::{NewUser, User};
use rocket::routes;
use rocket::{State, get, http::Status, post, serde::json::Json};
use tracing::instrument;

#[post("/", format = "json", data = "<user>")]
pub async fn create_user(
    ctx: &State<AppContext>,
    user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    let created = ctx.user_controller.create_user(user.into_inner()).await?;
    Ok(Json(created))
}

#[get("/<id>")]
#[instrument(skip(ctx))]
pub async fn get_user(ctx: &State<AppContext>, id: i32) -> Result<Json<User>, Status> {
    let user = ctx.user_controller.get_user(id).await?;
    Ok(Json(user))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, get_user]
}
