// src/api/user_management.rs
use actix_web::{post, get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{info, error};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct UserRequestModel {
    username: String,
    password: String,
    email: String,
}

impl User {
    fn new(username: String, password: String, email: String) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        User {
            id, username, password, email
        }
    }
}

#[get("/check")]
pub async fn check() -> impl Responder {
    info!("Check endpoint called");
    HttpResponse::Ok().json("Ok")
}

#[post("/register")]
pub async fn register_user(pool: web::Data<PgPool>, new_user: web::Json<UserRequestModel>) -> impl Responder {
    info!("Registering user: {}", new_user.username);

    let hashed_password = match bcrypt::hash(&new_user.password, 6) {
        Ok(pwd) => pwd,
        Err(e) => {
            error!("Failed to hash password: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to hash password");
        }
    };

    let user = User {
        id: COUNTER.fetch_add(1, Ordering::SeqCst),
        username: new_user.username.clone(),
        password: hashed_password,
        email: new_user.email.clone(),
    };

    let result = sqlx::query!(
        "INSERT INTO users (id, username, password, email) VALUES ($1, $2, $3, $4)",
        user.id as i32,
        user.username,
        user.password,
        user.email,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            info!("User {} registered successfully", user.username);
            HttpResponse::Ok().json("User Registered Successfully")
        }
        Err(e) => {
            error!("Failed to register user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to Register User")
        }
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, login_info: web::Json<UserRequestModel>) -> impl Responder {
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

