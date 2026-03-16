use crate::helpers::database::get_connection;
use crate::helpers::jwt::create_token;
use crate::helpers::rate_limit::LoginRateLimiter;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::services::user_service::{validate_user_credentials, UserValidationResult};
use actix_web::{get, post, web, web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use serde_json::json;
use tracing::warn;

/// JSON request body for API login.
#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

/// API login endpoint. Validates credentials and returns a JWT token on success.
/// Rate-limited per IP address.
#[post("/login")]
pub async fn login(
    http_req: HttpRequest,
    req: Json<LoginRequest>,
    limiter: web::Data<LoginRateLimiter>,
) -> impl Responder {
    // Check rate limit using IP address as key
    let ip = http_req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();
    let key = format!("api_login:{}", ip);

    match limiter.check(key.clone()) {
        Ok(()) => {} // Rate limit not exceeded
        Err(retry_after) => {
            warn!(
                ip = ip,
                path = http_req.path(),
                "Rate limit exceeded on API login attempt"
            );

            return HttpResponse::TooManyRequests()
                .insert_header(("Retry-After", retry_after.as_secs().to_string()))
                .json(json!({"error": "Too many login attempts. Please try again later."}));
        }
    }

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

/// Returns the authenticated user's profile as JSON. Requires a valid JWT token
/// (user ID is extracted by the JWT middleware).
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
