// src/api/mod.rs
pub mod user_management;
pub mod image_management;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(user_management::register_user)
        .service(user_management::login)
        .service(image_management::upload_image)
        .service(image_management::download_image);
}

