#![warn(async_fn_in_trait)]

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use twitter_clone_api::config::app::AppState;
use twitter_clone_api::dtos::user_dto::UserDTO;
use twitter_clone_api::http::controllers::find_tweet::find_tweet_by_id;
use twitter_clone_api::http::controllers::get_user_profile::get_user_profile;
use twitter_clone_api::http::controllers::post_tweet::post_tweet;
use twitter_clone_api::services::metrics;
use twitter_clone_api::services::metrics::UserMetricsService;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app_data = AppState::new().await;
    let app_url = app_data.config.app.url.clone();
    let app_port = app_data.config.app.port.parse::<u16>()?;

    app_data.user_repo.create(UserDTO {
        username: "daniel-reis".into(),
        biography: "hate rust".into(),
        birthdate: chrono::NaiveDate::from_ymd_opt(1999, 03, 08).unwrap()
    }).await?;

    let result = app_data.metrics.get_by_username("daniel-reis").await?;
    println!("{:?}", result);

    Ok(())    // println!("Web Server Online!");
    // println!("Listening on http://{}:{}", app_url, app_port);
    //
    // HttpServer::new(move || {
    //     App::new()
    //         .service(web::scope("/api")
    //             .service(get_user_profile)
    //             .service(post_tweet)
    //             .service(find_tweet_by_id)
    //         )
    //         .app_data(Data::new(app_data.clone()))
    // }).bind((app_url, app_port))?.run().await?;
    //
    // Ok(())
}