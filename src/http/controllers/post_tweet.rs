use actix_web::{HttpResponse, post, Responder, web};
use charybdis::types::Text;
use serde::Deserialize;
use serde_json::json;

use crate::config::app::AppState;
use crate::http::HttpError;
use crate::repositories::tweet_repository::TweetRepository;

#[derive(Deserialize)]
pub struct TweetRequestDTO {
    pub username: Text,
    pub content: String,
}

#[post("/tweet")]
async fn post_tweet(
    data: web::Data<AppState>,
    payload: web::Json<TweetRequestDTO>,
) -> Result<impl Responder, HttpError> {
    println!("{:?}", "fodase");
    let repository = TweetRepository::new(data.database.clone());
    let user = repository.tweet(payload.into_inner()).await;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(json!(user))),
        Err(e) => Err(e)
    }
}