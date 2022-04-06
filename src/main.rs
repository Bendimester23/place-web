mod canvas;

use redis::{Commands, ConnectionLike};
use actix_web::{App, HttpServer, main, web, get, post, Responder, HttpResponse};
use std::sync::Mutex;
use image::EncodableLayout;
use actix_web::middleware::Logger;
use actix_web::web::Bytes;
use serde::Deserialize;

struct AppData {
    canvas: Mutex<canvas::CanvasPicture>
}

#[derive(Deserialize)]
struct PlacePixelRequest {
    x: u32,
    y: u32,
    color: u8,
    key: String,
}

#[main]
async fn main() {
    println!("Starting web server on port 8080");

    let data = web::Data::new(AppData {
        canvas: Mutex::new(canvas::CanvasPicture::new(256, 256))
    });

    HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(data.clone())
                .service(index)
                .service(full)
                .service(place)
        }
    )
        .bind("0.0.0.0:8080").unwrap()
        .run().await.unwrap()
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}

#[post("/place")]
async fn place(body: web::Json<PlacePixelRequest>, data: web::Data<AppData>) -> impl Responder {
    if body.key != String::from("key") {
        HttpResponse::Forbidden()
    }

    let mut canvas = data.canvas.lock().unwrap();

    canvas.set_pixel(body.x, body.y, body.color).unwrap();

    HttpResponse::Created()
}

#[get("/full")]
async fn full(data: web::Data<AppData>) -> HttpResponse {
    let b = data.canvas.lock().unwrap().get_bytes();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(Bytes::copy_from_slice(b.as_bytes()))
}

fn tmp_redis_stuff() {
    // println!("Hello, world!");
    // let mut connection = redis::Client::open("redis://:rHCHiMuWiQaAMZeMB7Flr6zlvkmjlEdx@redis-13708.c293.eu-central-1-1.ec2.cloud.redislabs.com:13708/0").unwrap();
    //
    // println!("Connected to Redis");
    //
    // if !connection.check_connection() {
    //     println!("No connection");
    // }
    //
    // let _ : () = connection.set("my_key", 42).unwrap();
    //
    // let mut conn = connection.get_connection().unwrap();
    //
    //
    // let mut pubsub = conn.as_pubsub();
    //
    // pubsub.subscribe("place").unwrap();
    //
    // loop {
    //     let msg = pubsub.get_message().unwrap();
    //     let content: String = msg.get_payload().unwrap();
    //     let channel: String = msg.get_channel().unwrap();
    //
    //     println!("Got message on: {}, with content: {}", channel, content);
    // }
}
