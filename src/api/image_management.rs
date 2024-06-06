// src/api/image_management.rs
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{info, error};

static IMAGE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
struct Image {
    id: usize,
    filename: String,
    data: Vec<u8>,
}

impl Image {
    // Método para crear una nueva imagen con ID autoincrementable
    fn new(filename: String, data: Vec<u8>) -> Self {
        let id = IMAGE_COUNTER.fetch_add(1, Ordering::SeqCst);
        Image {
            id,
            filename,
            data,
        }
    }
}

#[post("/upload")]
pub async fn upload_image(pool: web::Data<PgPool>, mut payload: Multipart) -> impl Responder {
    let mut filename = String::new();
    let mut data = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        let name = content_disposition.get_name().unwrap();

        if name == "filename" {
            while let Some(chunk) = field.next().await {
                let chunk = chunk.unwrap();
                filename.push_str(std::str::from_utf8(&chunk).unwrap());
            }
        } else if name == "data" {
            while let Some(chunk) = field.next().await {
                let chunk = chunk.unwrap();
                data.extend_from_slice(&chunk);
            }
        }
    }

    let new_image = Image::new(filename, data);

    let result = sqlx::query!(
        "INSERT INTO images (id, filename, data) VALUES ($1, $2, $3)",
        new_image.id as i32,
        new_image.filename,
        new_image.data,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Image Uploaded Successfully"),
        Err(e) => {
            error!("Failed to upload image: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to Upload Image")
        }
    }
}

#[get("/download/{filename}")]
pub async fn download_image(pool: web::Data<PgPool>, filename: web::Path<String>) -> impl Responder {
    info!("Downloading image: {}", filename);

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
        Err(_) => {
            error!("Image not found: {}", filename);
            HttpResponse::NotFound().body("Image not found")
        }
    }
}

