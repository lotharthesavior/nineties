use actix_session::{Session, SessionExt};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, ResponseError};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::{AppState};
use crate::helpers::database::get_connection;
use crate::helpers::session::get_session_user;
use crate::helpers::template::load_template;
use crate::models::user::{User};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, name, password};
use crate::services::user_service::{prepare_password, validate_user_credentials, UserValidationResult};

#[get("")] // /admin - The Dashboard
pub async fn dashboard(data: web::Data<AppState>, session: Session) -> HttpResponse {
    let user: Option<User> = get_session_user(&session);
    let app_name = &data.app_name.lock().unwrap();

    HttpResponse::Ok().body(load_template(
        "admin/pages/dashboard.html",
        vec![
            ("name", app_name),
            ("user_name", &user.unwrap().name),
        ],
        None
    ))
}

#[get("/settings")]
pub async fn settings(_req: HttpRequest, data: web::Data<AppState>, session: Session) -> impl Responder {
    let user: Option<User> = get_session_user(&session);
    let app_name = &data.app_name.lock().unwrap();

    HttpResponse::Ok().body(load_template(
        "admin/pages/settings.html",
        vec![
            ("name", app_name),
            ("user_name", &user.unwrap().name)
        ],
        None
    ))
}

#[get("/profile")]
pub async fn profile(_req: HttpRequest, data: web::Data<AppState>, session: Session) -> impl Responder {
    let user: User = get_session_user(&session).unwrap();
    let app_name = &data.app_name.lock().unwrap();
    let user_name: String = user.name;
    let user_email: String = user.email;

    HttpResponse::Ok().body(load_template(
        "admin/pages/profile.html",
        vec![
            ("name", app_name),
            ("user_name", &user_name),
            ("user_email", &user_email)
        ],
        None
    ))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserForm {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordForm {
    current_email: String,
    old_password: String,
    new_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileResponse {
    data: UserForm,
}

#[post("/profile")]
pub async fn profile_post(
    form: web::Form<UserForm>,
    data: web::Data<AppState>,
    session: Session
) -> impl Responder {
    let user: User = get_session_user(&session).unwrap();

    let app_name = &data.app_name.lock().unwrap();
    let user_name: String = form.name.clone();
    let user_email: String = form.email.clone();

    let result = diesel::update(users.find(user.id))
        .set((email.eq(user_email.clone()), name.eq(user_name.clone())))
        .execute(&mut get_connection())
        .unwrap();

    if result == 0 {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"errors": {"server_error": "Failed to update user"}}));
    }

    let obj = ProfileResponse {
        data: UserForm {
            name: user_name.to_string(),
            email: user_email.to_string(),
        },
    };

    HttpResponse::Ok().json(obj)
}

#[post("/profile-password")]
pub async fn profile_password_post(
    form: web::Form<PasswordForm>,
    data: web::Data<AppState>,
    session: Session
) -> impl Responder {
    let user: User = get_session_user(&session).unwrap();

    let app_name = &data.app_name.lock().unwrap();
    let current_email: String = form.current_email.clone();
    let old_password: String = form.old_password.clone();
    let new_password: String = form.new_password.clone();

    let user_validation_result = validate_user_credentials(&current_email, &old_password);

    if user_validation_result != UserValidationResult::Valid {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"errors": {"server_error": "Invalid credentials"}}));
    }

    println!("new password: {}", new_password);
    let result = diesel::update(users.find(user.id))
        .set(password.eq(prepare_password(&*new_password)))
        .execute(&mut get_connection())
        .unwrap();

    if result == 0 {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"errors": {"server_error": "Failed to update user"}}));
    }

    HttpResponse::Ok().json(serde_json::json!({"success": "Password updated"}))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;
    use actix_session::{SessionMiddleware};
    use actix_session::storage::CookieSessionStore;
    use actix_web::{http, test, web, App};
    use actix_web::cookie::{Cookie, Key};
    use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
    use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
    use diesel_migrations::MigrationHarness;
    use serial_test::serial;
    use crate::{AppState};
    use crate::database::seeders::create_users::UserSeeder;
    use crate::database::seeders::traits::seeder::Seeder;
    use crate::helpers::database::get_connection;
    use crate::helpers::test::TestFinalizer;
    use crate::http::controllers::{admin_controller, auth_controller};
    use crate::http::middlewares::auth_middleware::AuthMiddleware;
    use crate::models::user::{User, MIGRATIONS};
    use crate::schema::users::dsl::*;
    use crate::services::user_service::{validate_user_credentials, UserValidationResult};

    fn prepare_test_db() -> PooledConnection<ConnectionManager<SqliteConnection>> {
        dotenv::from_filename(".env.test").ok();
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = get_connection();
        conn.revert_all_migrations(MIGRATIONS).expect("Failed to revert migrations");
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

        conn
    }

    fn seed_users_table() {
        let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = prepare_test_db();
        UserSeeder::execute(&mut conn).expect("Failed to seed users table");
    }

    #[serial]
    #[actix_web::test]
    async fn test_dashboard() {
        let _finalizer = TestFinalizer;

        seed_users_table();

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
                .service(auth_controller::signin_post)
                .service(
                    web::scope("/admin")
                        .service(admin_controller::dashboard)
                        .service(admin_controller::settings)
                        .wrap(AuthMiddleware)
                )
        ).await;

        let req1 = test::TestRequest::get()
            .uri("/admin")
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert_eq!(resp1.status(), http::StatusCode::FOUND);

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
            .cookie(parsed_cookie)
            .uri("/admin")
            .to_request();
        let resp3 = test::call_service(&app, req3).await;
        assert_eq!(resp3.status(), http::StatusCode::OK);
    }

    #[serial]
    #[actix_web::test]
    async fn test_settings() {
        let _finalizer = TestFinalizer;

        seed_users_table();

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
                .service(auth_controller::signin_post)
                .service(
                    web::scope("/admin")
                        .service(admin_controller::dashboard)
                        .service(admin_controller::settings)
                        .wrap(AuthMiddleware)
                )
        ).await;

        let req1 = test::TestRequest::get()
            .uri("/admin")
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert_eq!(resp1.status(), http::StatusCode::FOUND);

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
            .cookie(parsed_cookie)
            .uri("/admin")
            .to_request();
        let resp3 = test::call_service(&app, req3).await;
        assert_eq!(resp3.status(), http::StatusCode::OK);
    }

    #[serial]
    #[actix_web::test]
    async fn test_profile_data() {
        let _finalizer = TestFinalizer;

        seed_users_table();

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
                .service(auth_controller::signin_post)
                .service(
                    web::scope("/admin")
                        .service(admin_controller::profile)
                        .service(admin_controller::profile_post)
                        .wrap(AuthMiddleware)
                )
        ).await;

        let req1 = test::TestRequest::post()
            .uri("/signin")
            .set_form(&[("email", "jekyll@example.com"), ("password", "password")])
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert_eq!(resp1.status(), http::StatusCode::FOUND);

        // Let's get the cookie from the last request here and repeat it!
        let headers = resp1.headers().clone();
        let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
        let parsed_cookie = Cookie::parse_encoded(cookie_header).unwrap();

        let req3 = test::TestRequest::get()
            .cookie(parsed_cookie.clone())
            .uri("/admin/profile")
            .to_request();
        let resp3 = test::call_service(&app, req3).await;
        assert_eq!(resp3.status(), http::StatusCode::OK);

        let new_email = "hyde@example.com";

        let req4 = test::TestRequest::post()
            .cookie(parsed_cookie)
            .uri("/admin/profile")
            .set_form(&[("name", "Hyde"), ("email", new_email)])
            .to_request();
        let resp4 = test::call_service(&app, req4).await;
        assert_eq!(resp4.status(), http::StatusCode::OK);

        let user = users
            .filter(email.eq(new_email))
            .load::<User>(&mut get_connection())
            .expect("Failed to load users");

        assert_eq!(user.len(), 1);
    }

    #[serial]
    #[actix_web::test]
    async fn test_profile_password() {
        let _finalizer = TestFinalizer;

        seed_users_table();

        let user_email = "jekyll@example.com";

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
                .service(auth_controller::signin_post)
                .service(
                    web::scope("/admin")
                        .service(admin_controller::profile)
                        .service(admin_controller::profile_password_post)
                        .wrap(AuthMiddleware)
                )
        ).await;

        let req1 = test::TestRequest::post()
            .uri("/signin")
            .set_form(&[
                ("email", user_email),
                ("password", "password"),
            ])
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert_eq!(resp1.status(), http::StatusCode::FOUND);

        // Let's get the cookie from the last request here and repeat it!
        let headers = resp1.headers().clone();
        let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
        let parsed_cookie = Cookie::parse_encoded(cookie_header).unwrap();

        let req3 = test::TestRequest::get()
            .cookie(parsed_cookie.clone())
            .uri("/admin/profile")
            .to_request();
        let resp3 = test::call_service(&app, req3).await;
        assert_eq!(resp3.status(), http::StatusCode::OK);

        let req4 = test::TestRequest::post()
            .cookie(parsed_cookie)
            .uri("/admin/profile-password")
            .set_form(&[
                ("current_email", user_email),
                ("old_password", "password"),
                ("new_password", "new-password"),
            ])
            .to_request();
        let resp4 = test::call_service(&app, req4).await;
        assert_eq!(resp4.status(), http::StatusCode::OK);
        assert_eq!(validate_user_credentials(user_email, "new-password"), UserValidationResult::Valid);
    }
}
