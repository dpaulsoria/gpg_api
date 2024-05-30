use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// Estructura de datos
#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
    price: f64,
}

// Endpoint
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_item() -> impl Responder {
    let item = Item {
        id: 1,
        name: "Example Item".to_string(),
        price: 9.99,
    };

    HttpResponse::Ok().json(item)
}

async fn create_item(item: web::Json<Item>) -> impl Responder {
    HttpResponse::Ok().json(item.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/item", web::get().to(get_item))
            .route("/item", web::post().to(create_item))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
