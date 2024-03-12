use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use crate::config::app::AppState;
use crate::http::controllers::get_user_profile::get_user_profile;

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
            )
            .app_data(Data::new(app_data.clone()))
    }).bind((app_url, app_port))?.run().await
}