// src/api/user_management.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    email: String,
}

#[post("/register")]
pub async fn register_user(pool: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    let hashed_password = bcrypt::hash(&user.password, 6).expect("Failed to hash password");

    let result = sqlx::query!(
        "INSERT INTO users (id, username, password, email) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        user.username,
        hashed_password,
        user.email,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User Registered Successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to Register User"),
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, login_info: web::Json<User>) -> impl Responder {
    let row = sqlx::query!(
        "SELECT password FROM users WHERE username = $1",
        login_info.username
    )
    .fetch_one(pool.get_ref())
    .await;

    match row {
        Ok(record) => {
            let is_valid = bcrypt::verify(&login_info.password, &record.password).unwrap_or(false);
            if is_valid {
                HttpResponse::Ok().json("Login successful")
            } else {
                HttpResponse::Unauthorized().json("Invalid username or password")
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid username or password"),
    }
}
