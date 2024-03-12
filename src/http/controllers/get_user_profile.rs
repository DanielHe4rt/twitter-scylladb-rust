use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;
use crate::config::app::AppState;
use crate::http::HttpError;

use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

#[get("/users/{username}")]
async fn get_user_profile(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<impl Responder, HttpError> {
    println!("{:?}", "fodase");
    let repository = UserRepository::new(data.database.clone());
    let user = repository.get_user(username.into_inner()).await;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(json!(user))),
        Err(e) => Err(e)
    }
}