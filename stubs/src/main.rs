use actix_web::{web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use dotenv::dotenv;
use std::{env, fs};
use std::path::PathBuf;
use std::sync::Mutex;
use actix_web::cookie::Key;
use actix_web::middleware::NormalizePath;
use diesel::SqliteConnection;
use diesel_migrations::MigrationHarness;
use tokio::io::{AsyncBufReadExt};
use crate::database::seeders::create_users::{Seeder, UserSeeder};

mod helpers;
mod routes;
mod http {
    pub mod middlewares {
        pub mod auth_middleware;
    }

    pub mod controllers {
        pub mod home_controller;
        pub mod auth_controller;
        pub mod admin_controller;
    }
}

mod database {
    pub mod seeders {
        pub mod create_users;
    }
}

mod models {
    pub mod user;
}

mod schema;

mod console {
    pub mod development;
}

#[derive(Debug)]
struct AppState {
    app_name: Mutex<String>,
    user_id: Mutex<Option<i32>>,
}

fn check_app_health() {
    println!("checking stuff");
    if !fs::exists(PathBuf::from(".env")).unwrap() {
        fs::copy(PathBuf::from(".env.example"), PathBuf::from(".env"))
            .expect("Failed to copy .env.example to .env");
    }

    if !fs::exists(PathBuf::from("database/database.sqlite")).unwrap() {
        let mut conn: SqliteConnection = helpers::get_connection();
        conn.run_pending_migrations(models::user::MIGRATIONS).expect("Failed to run migrations");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    check_app_health();

    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let app_url: String = env::var("APP_URL")
        .expect("APP_URL must be set");
    let app_port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid u16");

    if args.len() > 1 {
        match args[1].as_str() {
            "serve" => {
                let secret_key = Key::from(env::var("SECRET_KEY")
                    .expect("SECRET_KEY must be set")
                    .as_bytes());

                HttpServer::new(move || {
                    App::new()
                        .wrap(SessionMiddleware::new(
                            CookieSessionStore::default(),
                            secret_key.clone(),
                        ))
                        .wrap(NormalizePath::trim())
                        .app_data(web::Data::new(AppState {
                            app_name: Mutex::from(env::var("APP_NAME").unwrap_or_else(|_| "".to_string())),
                            user_id: Mutex::from(None),
                        }))
                        .configure(routes::config)
                })
                .bind((app_url, app_port))?
                .run()
                .await
            }
            "develop" => {
                console::development::run_development().await
            }
            "migrate" => {
                println!("Running migrations...");
                let mut conn: SqliteConnection = helpers::get_connection();
                conn.run_pending_migrations(models::user::MIGRATIONS).expect("Failed to run migrations");
                Ok(())
            }
            "seed" => {
                println!("Running seeders...");
                let _ = UserSeeder::execute(&mut helpers::get_connection()).expect("Failed to seed users table");
                Ok(())
            }
            _ => {
                eprintln!("Unknown command");
                Ok(())
            }
        }
    } else {
        eprintln!("No command provided");
        Ok(())
    }
}