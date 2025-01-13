use crate::helpers::{get_session_message, is_authenticated, load_template};
use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;
use actix_session::Session;

#[get("/")]
pub async fn home(data: web::Data<AppState>, session: Session) -> impl Responder {
    let user_authenticated: &str = if is_authenticated(&session) { "true" } else { "false" };
    let app_name = &data.app_name.lock().unwrap();

    HttpResponse::Ok().body(
        load_template("home.html", vec![
            ("name", app_name),
            ("user_authenticated", &user_authenticated),
            ("session_message", &get_session_message(&session, false).1)
        ], Option::from(vec!["src/resources/css/styles.css", "src/resources/js/script.js"]))
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use actix_web::{http, test, web, App};
    use crate::{AppState};
    use crate::http::controllers::home_controller;

    #[actix_web::test]
    async fn test_home() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    app_name: Mutex::from(String::from("My App Name")),
                    user_id: Mutex::from(None),
                }))
                .service(home_controller::home)
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
