mod canvas;
mod config;
mod routes;

use actix_web::{App, HttpServer, web};
use std::sync::{Mutex, RwLock};
use actix_web::middleware::Logger;
use crate::config::AppConfig;

pub struct AppData {
    canvas: RwLock<canvas::CanvasPicture>,
    config: AppConfig,
    online_players: RwLock<u32>,
}

const LOGIN: &'static str = include_str!("../static/login.html");

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let conf = config::load();
    println!("Starting web server on port 8080");

    let data = web::Data::new(AppData {
        canvas: RwLock::new(canvas::CanvasPicture::new(conf.canvas_width, conf.canvas_height)),
        config: conf,
        online_players: RwLock::new(0)
    });

    HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(data.clone())
                .service(routes::index)
                .service(routes::full)
                .service(routes::place)
                .service(routes::map)
                .service(routes::rect)
                .service(routes::test)
                .service(routes::save)
                .service(routes::random_ad)
                .service(routes::random_motd)
                .service(routes::set_online_players)
                .service(routes::get_online_players)
        }
    )
        .bind("0.0.0.0:8080").unwrap()
        .run().await.unwrap()
}
