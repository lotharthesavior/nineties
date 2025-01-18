use std::error::Error;
use actix_session::Session;
use crate::helpers::{get_from_form_body, get_session_message, is_authenticated, load_template};
use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use crate::{helpers, AppState};
use crate::models::user::{User};
use crate::schema::users::dsl::*;

#[get("/signin")]
pub async fn signin(data: web::Data<AppState>, session: Session) -> impl Responder {
    let app_name = &data.app_name.lock().unwrap();

    if is_authenticated(&session) {
        return HttpResponse::Found().insert_header(("Location", "/admin")).finish();
    }

    let session_message: (String, String) = get_session_message(&session, true);

    HttpResponse::Ok().body(load_template(
        "signin.html",
        vec![
            ("name", app_name),
            ("session_message", &*session_message.1)
        ],
        None
    ))
}

#[get("/signout")]
pub async fn signout(session: Session) -> impl Responder {
    session.remove("user_id");
    session.insert("message", "You have been signed out").unwrap();

    HttpResponse::Found().insert_header(("Location", "/")).finish()
}

#[post("/signin")]
pub async fn signin_post(req_body: String, session: Session) -> impl Responder {
    let conn = &mut helpers::get_connection();

    let email_param: String = get_from_form_body("email".to_string(), req_body.clone());
    let password_param: String = get_from_form_body("password".to_string(), req_body);

    if email_param.is_empty() || password_param.is_empty() {
        session.insert("message", serde_json::json!({
            "error": "Email and password are required",
            "success": ""
        })).unwrap();

        return HttpResponse::Found().insert_header(("Location", "/signin")).finish();
    }

    let user = users
        .filter(email.eq(&email_param))
        .load::<User>(conn)
        .expect("Failed to load users");

    if user.len() == 0 {
        session.insert("message", serde_json::json!({
            "error": "Invalid credentials",
            "success": ""
        })).unwrap();

        return HttpResponse::Found().insert_header(("Location", "/signin")).finish();
    }

    let user: &User = user.first().unwrap();
    let parsed_hash = PasswordHash::new(&user.password);
    if parsed_hash.is_err() {
        println!("Invalid credentials: Couldn't parse password hash");
        session.insert("message", serde_json::json!({
            "error": "Invalid credentials",
            "success": ""
        })).unwrap();

        return HttpResponse::Found().insert_header(("Location", "/signin")).finish();
    }

    let password_verified: bool = Argon2::default()
        .verify_password((&*password_param).as_ref(), &parsed_hash.unwrap())
        .is_ok();

    if password_verified {
        session.insert("user_id", user.id).unwrap();

        HttpResponse::Found().insert_header(("Location", "/admin")).finish()
    } else {
        session.insert("message", serde_json::json!({
            "error": "Invalid credentials",
            "success": ""
        })).unwrap();

        HttpResponse::Found().insert_header(("Location", "/signin")).finish()
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;
    use actix_session::{Session, SessionMiddleware};
    use actix_session::storage::CookieSessionStore;
    use actix_web::{http, test, web, App, HttpRequest, HttpResponse};
    use actix_web::cookie::{Cookie, Key};
    use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
    use diesel_migrations::MigrationHarness;
    use crate::{helpers, AppState};
    use crate::database::seeders::create_users::UserSeeder;
    use crate::database::seeders::traits::seeder::Seeder;
    use crate::http::controllers::auth_controller;
    use crate::http::middlewares::auth_middleware::AuthMiddleware;
    use crate::models::user::{MIGRATIONS};
    use crate::schema::users::dsl::users;
    use crate::schema::users::{id};

    fn prepare_test_db() -> SqliteConnection {
        dotenv::from_filename(".env.test").ok();
        let mut conn: SqliteConnection = helpers::get_connection();
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
        conn
    }

    fn seed_users_table() {
        let mut conn: SqliteConnection = prepare_test_db();
        UserSeeder::execute(&mut conn).expect("Failed to seed users table");
    }

    #[actix_web::test]
    async fn test_signin_route() {
        let mut conn: SqliteConnection = prepare_test_db();
        let all_users: Vec<i32> = users.select(id).load::<i32>(&mut conn).unwrap();
        let user_id: i32 = all_users[0];

        let secret_key = Key::from(env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set")
            .as_bytes());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    app_name: Mutex::from(env::var("APP_NAME").unwrap_or_else(|_| "".to_string())),
                    user_id: Mutex::from(None),
                }))
                .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                ))
                .service(auth_controller::signin)
                .service(auth_controller::signin_post)
                .service(auth_controller::signout)
                .service(
                    web::resource("/check-data")
                        .route(web::get().to({
                            let user_id: i32 = user_id.clone();
                            move |req: HttpRequest, session: Session| async move {
                                let session_user_id: i32 = session.get::<i32>("user_id").unwrap_or(Some(0)).unwrap_or(0);
                                if user_id == session_user_id {
                                    HttpResponse::Ok()
                                } else {
                                    HttpResponse::BadRequest()
                                }
                            }
                        }))
                        .wrap(AuthMiddleware)
                )
        ).await;

        let req1 = test::TestRequest::get()
            .uri("/signin")
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert_eq!(resp1.status(), http::StatusCode::OK);

        let req2 = test::TestRequest::post()
            .uri("/signin")
            .set_form(&[("email", "jekyll@example.com"), ("password", "password")])
            .to_request();
        let resp2 = test::call_service(&app, req2).await;
        assert_eq!(resp2.status(), http::StatusCode::FOUND);

        // Let's get the cookie from the last request here and repeat it!
        let headers = resp2.headers().clone();
        let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
        let parsed_cookie = Cookie::parse_encoded(cookie_header).unwrap();

        let req3 = test::TestRequest::get()
            .cookie(parsed_cookie.clone())
            .uri("/check-data")
            .to_request();
        let resp3 = test::call_service(&app, req3).await;
        assert_eq!(resp3.status(), http::StatusCode::OK);

        // Now we logout and check if the session is destroyed
        let req4 = test::TestRequest::get()
            .cookie(parsed_cookie)
            .uri("/signout")
            .to_request();
        let resp4 = test::call_service(&app, req4).await;
        assert_eq!(resp4.status(), http::StatusCode::FOUND);

        // Let's get the cookie from the last request here and repeat it!
        let headers = resp4.headers().clone();
        let cookie_header2 = headers.get("set-cookie").unwrap().to_str().unwrap();
        let parsed_cookie2 = Cookie::parse_encoded(cookie_header2).unwrap();

        let req5 = test::TestRequest::get()
            .cookie(parsed_cookie2)
            .uri("/check-data")
            .to_request();
        let resp5 = test::call_service(&app, req5).await;
        assert_eq!(resp5.status(), http::StatusCode::FOUND);
    }
}
