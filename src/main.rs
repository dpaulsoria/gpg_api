// src/main.rs
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tracing::{info, error};
use tracing_subscriber;

mod api;
mod db;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    // Cargar variables de entorno
    dotenv().ok();

    // Obtener la URL de la base de datos desde las variables de entorno
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Inicializar la conexión a la base de datos
    let pool = db::init_db(&database_url).await.expect("Failed to create pool.");

    info!("Starting server at http://localhost:8080");

    // Iniciar el servidor
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
