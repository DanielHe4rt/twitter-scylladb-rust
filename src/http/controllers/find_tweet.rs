use actix_web::{get, HttpResponse, Responder, web};
use charybdis::types::Uuid;
use serde_json::json;

use crate::config::app::AppState;
use crate::repositories::tweet_repository::TweetRepository;

#[get("/tweet/{tweetId}")]
async fn find_tweet_by_id(
    data: web::Data<AppState>,
    tweet_id: web::Path<Uuid>,
) -> Result<impl Responder, crate::Error> {
    let repository = TweetRepository::new(data.database.clone());
    let user = repository.find_tweet(tweet_id.into_inner()).await;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(json!(user))),
        Err(e) => Err(e)
    }
}