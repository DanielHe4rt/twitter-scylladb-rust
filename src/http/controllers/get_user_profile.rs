use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;

use crate::config::app::AppState;

#[get("/users/{username}")]
async fn get_user_profile(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<impl Responder, crate::Error> {

    let username = username.into_inner();

    let user = data.user_repo.get_user(username.clone()).await?;
    let followers = data.user_repo.get_followers(username.clone()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "user": user,
        "followers": followers
    })))
}