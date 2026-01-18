use crate::routes;
use crate::websocket::server::WsServer;
use crate::AppState;
use actix::prelude::*;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};
use std::env;
use std::io;
use std::sync::Mutex;

pub async fn run(app_url: String, app_port: u16) -> io::Result<()> {
    crate::check_database_health();

    let secret_key = Key::from(
        env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set")
            .as_bytes(),
    );

    let ws_server = WsServer::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(AppState {
                app_name: Mutex::from(env::var("APP_NAME").unwrap_or_else(|_| "".to_string())),
                _user_id: Mutex::from(None),
            }))
            .app_data(web::Data::new(ws_server.clone()))
            .configure(routes::config)
    })
    .bind((app_url, app_port))?
    .run()
    .await
}
