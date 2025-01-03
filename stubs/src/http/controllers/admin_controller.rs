use actix_session::{Session, SessionExt};
use crate::helpers::{load_template};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, ResponseError};
use diesel::{QueryDsl, RunQueryDsl};
use crate::{helpers, AppState};
use crate::models::user::{User};
use crate::schema::users::dsl::users;

#[get("")] // /admin - The Dashboard
pub async fn dashboard(data: web::Data<AppState>, session: Session) -> HttpResponse {
    let user_id: i32 = session.get::<i32>("user_id").unwrap_or(Some(0)).unwrap_or(0);
    let app_name = &data.app_name.lock().unwrap();

    let user = users.find(user_id).first::<User>(&mut helpers::get_connection());
    if user.is_err() {
        return HttpResponse::Found().insert_header(("Location", "/signin")).finish();
    }

    HttpResponse::Ok().body(load_template("admin/dashboard.html", vec![("name", app_name), ("user_name", &user.unwrap().name)]))
}

#[get("/settings")]
pub async fn settings(_req: HttpRequest, data: web::Data<AppState>, session: Session) -> impl Responder {
    let app_name = &data.app_name.lock().unwrap();

    let user_id = session.get::<i32>("user_id").unwrap_or(Some(0)).unwrap_or(0);
    let user = users.find(user_id).first::<User>(&mut helpers::get_connection());
    if user.is_err() {
        return HttpResponse::Found().insert_header(("Location", "/signin")).finish();
    }

    HttpResponse::Ok().body(load_template("admin/settings.html", vec![("name", app_name), ("user_name", &user.unwrap().name)]))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;
    use actix_session::{Session, SessionMiddleware};
    use actix_session::storage::CookieSessionStore;
    use actix_web::{http, test, web, App};
    use actix_web::cookie::{Cookie, Key};
    use diesel::{SqliteConnection};
    use diesel_migrations::MigrationHarness;
    use crate::{helpers, AppState};
    use crate::database::seeders::create_users::{Seeder, UserSeeder};
    use crate::http::controllers::{admin_controller, auth_controller};
    use crate::http::middlewares::auth_middleware::AuthMiddleware;
    use crate::models::user::{MIGRATIONS};

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
    async fn test_dashboard() {
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

    #[actix_web::test]
    async fn test_settings() {
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
}