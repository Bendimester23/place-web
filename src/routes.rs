use std::fs::File;
use std::io::Write;
use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, web, get, post};
use actix_web::web::Bytes;
use image::EncodableLayout;
use serde::{Deserialize, Serialize};

const INDEX: &'static str = include_str!("../static/index.html");

/* Request bodies */
#[derive(Deserialize, Serialize)]
pub struct PlacePixelRequest {
    x: u32,
    y: u32,
    color: u8,
    key: String,
}

#[derive(Deserialize)]
pub struct SaveRequest {
    key: String
}

#[derive(Deserialize, Serialize)]
pub struct RectRequest {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: u8,
    key: String,
}

#[derive(Deserialize, Serialize)]
pub struct SetPlayersRequest {
    count: u32,
    key: String
}

#[get("/")]
pub async fn index() -> impl Responder {
//    NamedFile::open("./static/index.html")
    HttpResponse::Ok().content_type("text/html").body(INDEX)
}

#[get("/map")]
pub async fn map(data: web::Data<crate::AppData>) -> impl Responder {
    return match data.canvas.read() {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(canvas) => HttpResponse::Ok().body(Bytes::copy_from_slice(canvas.get_data()))
    }
}


#[post("/place")]
pub async fn place(body: web::Json<PlacePixelRequest>, data: web::Data<crate::AppData>) -> impl Responder {
    if body.key != data.config.place_key {
        return HttpResponse::Forbidden().finish();
    }

    return match data.canvas.write() {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(mut canvas) => {
            canvas.set_pixel(body.x, body.y, body.color).unwrap();
            HttpResponse::Created().body("success")
        }
    }
}

#[get("/test")]
pub async fn test() -> impl Responder {
    //redis.send(Command(resp_array!["SET", "cucc", "az"])).await.unwrap();

    //redis.send(Command(resp_array!["GET", "cucc"])).await.unwrap()
    "igen"
}

#[get("/full")]
pub async fn full(data: web::Data<crate::AppData>) -> HttpResponse {
    return match data.canvas.read() {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(b) => {
            HttpResponse::Ok()
                .content_type("image/png")
                .body(Bytes::copy_from_slice(b.get_bytes().as_bytes()))
        }
    }
}

#[get("/save")]
pub async fn save(data: web::Data<crate::AppData>, req: web::Query<SaveRequest>) -> impl Responder {
    if req.key != data.config.place_key {
        HttpResponse::Forbidden().body("Invalid key");
    }

    return match data.canvas.read() {
        Ok(canvas) => {
            return match File::create("./map") {
                Ok(mut f) => {
                    f.write_all(canvas.get_data()).unwrap();
                    HttpResponse::Ok().body("success")
                }
                Err(e) => {
                    HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    };
}

#[get("/ad")]
pub async fn random_ad(d: web::Data<crate::AppData>) -> impl Responder {
    let all: f32 = d.config.ads.len() as f32;
    let choice: usize = (rand::random::<f32>() * all) as usize;
    let motd = &d.config.ads.as_slice()[choice];

    HttpResponse::Ok().content_type("text/plain").body(Bytes::copy_from_slice(motd.as_bytes()))
}

#[get("/motd")]
pub async fn random_motd(d: web::Data<crate::AppData>) -> impl Responder {
    let all: f32 = d.config.motds.len() as f32;
    let choice: usize = (rand::random::<f32>() * all) as usize;
    let ad = &d.config.motds.as_slice()[choice];

    HttpResponse::Ok().content_type("text/plain").body(Bytes::copy_from_slice(ad.as_bytes()))
}

#[post("/players")]
pub async fn set_online_players(data: web::Data<crate::AppData>, body: web::Json<SetPlayersRequest>) -> impl Responder {
    if data.config.place_key != body.key {
        return HttpResponse::Forbidden().body("invalid key")
    }

    return match data.online_players.write() {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(mut count) => {
            *count = body.count;
            HttpResponse::Ok().body("success")
        }
    }
}

#[get("/players")]
pub async fn get_online_players(data: web::Data<crate::AppData>) -> impl Responder {
    return match data.online_players.read() {
        Ok(count) => HttpResponse::Ok().body(count.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/rect")]
pub async fn rect(data: web::Data<crate::AppData>, body: web::Json<RectRequest>) -> impl Responder {
    if body.key != data.config.place_key {
        return HttpResponse::Forbidden().finish();
    }

    return match data.canvas.write() {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(mut canvas) => {
            match canvas.fill_rect(body.x, body.y, body.width, body.height, body.color) {
                Err(e) => HttpResponse::BadRequest().body(e.to_string()),
                Ok(_a) => HttpResponse::Created().body("success")
            }
        }
    }
}
