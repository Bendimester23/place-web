mod canvas;
//mod socket;

use actix_web::{App, HttpServer, main, web, get, post, Responder, HttpResponse};
use std::sync::Mutex;
use actix::{Actor, Addr, Handler};
use image::EncodableLayout;
use actix_web::middleware::Logger;
use actix_web::web::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::socket::Lobby;

struct AppData {
    canvas: Mutex<canvas::CanvasPicture>,
    //notifs: Mutex<Vec<u8>>,
}

#[derive(Deserialize, Serialize)]
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
        canvas: Mutex::new(canvas::CanvasPicture::new(256, 256)),
        //notifs: Mutex::new(vec![0, 1])
    });

    //let lobby = socket::Lobby::default().start();

    HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(data.clone())
                //.app_data(lobby.clone())
                .service(index)
                .service(full)
                .service(place)
                //.service(socket::start_connection)
        }
    )
        .bind("0.0.0.0:8080").unwrap()
        .run().await.unwrap()
}

#[get("/")]
async fn index(data: web::Data<AppData>) -> impl Responder {
    let mut notifs = data.notifs.lock().unwrap();

    notifs.push(1);

    "Hello World"
}

#[post("/place")]
async fn place(body: web::Json<PlacePixelRequest>, data: web::Data<AppData>/*, srv: web::Data<Addr<Lobby>>*/) -> impl Responder {
    if body.key != String::from("key") {
        return HttpResponse::Forbidden().finish();
    }

    // let msg = socket::BroadcastMessage{
    //     id: Uuid::parse_str("470bb217-ffa7-43d8-a0cc-b3d30421d1werfw").unwrap(),
    //     msg: json!(body.0),
    //     room_id:  String::from("place")
    // };
    // srv.do_send(msg);

    let mut canvas = data.canvas.lock().unwrap();

    canvas.set_pixel(body.x, body.y, body.color).unwrap();

    HttpResponse::Created().finish()
}

#[get("/full")]
async fn full(data: web::Data<AppData>) -> HttpResponse {
    let b = data.canvas.lock().unwrap().get_bytes();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(Bytes::copy_from_slice(b.as_bytes()))
}
