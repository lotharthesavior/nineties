use actix_web::{web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use dotenv::dotenv;
use std::{env, fs};
use std::path::PathBuf;
use std::process::exit;
use std::sync::Mutex;
use actix_web::cookie::Key;
use actix_web::middleware::NormalizePath;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{IntoSql, SqliteConnection};
use diesel_migrations::MigrationHarness;
use tokio::io::{AsyncBufReadExt};
use crate::database::seeders::create_users::{UserSeeder};
use crate::database::seeders::traits::seeder::Seeder;
use crate::helpers::database::get_connection;

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

        pub mod traits {
            pub mod seeder;
        }
    }
}

mod models {
    pub mod user;
}

mod schema;

mod console {
    pub mod development;
}

mod helpers {
    pub mod session;
    pub mod database;
    pub mod form;
    pub mod general;
    pub mod template;
    pub mod test;
}

mod services {
    pub mod user_service;
}

#[derive(Debug)]
struct AppState {
    app_name: Mutex<String>,
    user_id: Mutex<Option<i32>>,
}

fn check_app_health() {
    println!("Checking App's Health.");
    if !fs::exists(PathBuf::from(".env")).unwrap() {
        fs::copy(PathBuf::from(".env.example"), PathBuf::from(".env"))
            .expect("Failed to copy .env.example to .env");
    }
}

fn check_database_health() {
    println!("Checking Database Health.");
    let database: String = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "database/database.sqlite".to_string());

    if !fs::exists(PathBuf::from(database)).unwrap() {
        println!("Database file not found. Please run `cargo run migrate` to create the database.");
        exit(1);
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

    let mut command: &str = "serve";
    if args.len() > 1 {
        command = args[1].as_str();
    }

    match command {
        "serve" => {
            check_database_health();

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
            check_database_health();
            console::development::run_development().await
        }
        "migrate" => {
            println!("Starting migration procedure.");

            if args.contains(&"--fresh".to_string()) {
                println!("Reverting all migrations...");
                let database: String = env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "database/database.sqlite".to_string());
                fs::remove_file(database).expect("Failed to remove database file");
            }

            println!("Running migrations...");
            let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = get_connection();
            conn.run_pending_migrations(models::user::MIGRATIONS)
                .expect("Failed to run migrations");

            if args.contains(&"--seed".to_string()) {
                println!("Running seeders...");
                let _ = UserSeeder::execute(&mut get_connection())
                    .expect("Failed to seed users table");
            }

            Ok(())
        }
        "seed" => {
            println!("Running seeders...");
            let _ = UserSeeder::execute(&mut get_connection())
                .expect("Failed to seed users table");
            Ok(())
        }
        _ => {
            eprintln!("Unknown command");
            Ok(())
        }
    }
}