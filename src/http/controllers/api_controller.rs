use crate::helpers::database::get_connection;
use crate::helpers::jwt::create_token;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::services::user_service::{validate_user_credentials, UserValidationResult};
use actix_web::{get, post, web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(req: Json<LoginRequest>) -> impl Responder {
    match validate_user_credentials(&req.email, &req.password) {
        UserValidationResult::Valid => {
            let conn = &mut get_connection();
            let user_vec: Vec<User> = users
                .filter(email.eq(&req.email))
                .load(conn)
                .expect("Failed to load user");
            if let Some(user) = user_vec.first() {
                match create_token(user.id) {
                    Ok(token) => HttpResponse::Ok().json(json!({"token": token})),
                    Err(_) => HttpResponse::InternalServerError()
                        .json(json!({"error": "Failed to generate token"})),
                }
            } else {
                HttpResponse::Unauthorized().json(json!({"error": "User not found"}))
            }
        }
        _ => HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"})),
    }
}

#[get("/profile")]
pub async fn profile(req: HttpRequest) -> impl Responder {
    if let Some(&user_id) = req.extensions().get::<i32>() {
        let conn = &mut get_connection();
        match users.find(user_id).first::<User>(conn) {
            Ok(user) => HttpResponse::Ok().json(&user),
            Err(_) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        }
    } else {
        HttpResponse::Unauthorized().json(json!({"error": "No authenticated user"}))
    }
}
