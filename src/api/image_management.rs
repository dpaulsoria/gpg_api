// src/api/image_management.rs
use actix_web::{post, get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct Image {
    filename: String,
    data: Vec<u8>,
}

#[post("/upload")]
pub async fn upload_image(
    pool: web::Data<PgPool>, 
    image: web::Json<Image>
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO images (filename, data) VALUES ($1, $2)",
        image.filename,
        image.data,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Image Uploaded Successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to Upload Image"),
    }
}

#[get("/download/{filename}")]
pub async fn download_image(
    pool: web::Data<PgPool>, 
    filename: web::Path<String>
) -> impl Responder {
    let row = sqlx::query!(
        "SELECT data FROM images WHERE filename = $1", 
        filename.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await;

    match row {
        Ok(record) => {
            let data = record.data;
            HttpResponse::Ok().body(data)
        },
        Err(_) => HttpResponse::NotFound().body("Image not found"),
    }
}

