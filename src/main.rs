use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use crate::config::app::AppState;
use crate::http::controllers::find_tweet::find_tweet_by_id;
use crate::http::controllers::get_user_profile::get_user_profile;
use crate::http::controllers::post_tweet::post_tweet;

mod models;

mod config;
mod http;
mod repositories;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppState::new().await;
    let app_url = app_data.clone().config.app.url;
    let app_port = app_data.clone().config.app.port.parse::<u16>().unwrap();


    println!("Web Server Online!");
    println!("Listening on http://{}:{}", app_url, app_port);
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api")
                .service(get_user_profile)
                .service(post_tweet)
                .service(find_tweet_by_id)
            )
            .app_data(Data::new(app_data.clone()))
    }).bind((app_url, app_port))?.run().await
}