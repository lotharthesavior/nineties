use dotenv::dotenv;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Mutex;
use std::{env, fs};

mod routes;
mod http {
    pub mod middlewares {
        pub mod auth_middleware;
        pub mod jwt_middleware;
    }

    pub mod controllers {
        pub mod admin_controller;
        pub mod api_controller;
        pub mod auth_controller;
        pub mod home_controller;
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

mod helpers {
    pub mod csrf;
    pub mod database;
    pub mod form;
    pub mod general;
    pub mod jwt;
    pub mod session;
    pub mod template;
    pub mod test;
}

mod services {
    pub mod user_service;
}

mod commands;
pub mod websocket;
#[derive(Debug)]
pub struct AppState {
    app_name: Mutex<String>,
    _user_id: Mutex<Option<i32>>,
}

fn check_app_health() {
    println!("Checking App's Health.");
    if !fs::exists(PathBuf::from(".env")).unwrap() {
        fs::copy(PathBuf::from(".env.example"), PathBuf::from(".env"))
            .expect("Failed to copy .env.example to .env");
    }
}

pub fn check_database_health() {
    println!("Checking Database Health.");
    let database: String =
        env::var("DATABASE_URL").unwrap_or_else(|_| "database/database.sqlite".to_string());

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
    let app_url: String = env::var("APP_URL").expect("APP_URL must be set");
    let app_port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid u16");

    let mut command: &str = "serve";
    if args.len() > 1 {
        command = args[1].as_str();
    }

    match command {
        "serve" => commands::serve::run(app_url.clone(), app_port).await,
        "develop" => {
            check_database_health();
            commands::develop::run_development().await
        }
        "migrate" => commands::migrate::run(&args),
        "seed" => commands::seed::run(),
        _ => {
            eprintln!("Unknown command");
            Ok(())
        }
    }
}
