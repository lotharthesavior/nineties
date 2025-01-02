use std::env;
use actix_session::Session;
use tera::Tera;
use tera::Context;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use diesel::{Connection, SqliteConnection};

pub fn load_template(template: &str, params: Vec<(&str, &str)>) -> String {
    let tera = match Tera::new("src/resources/views/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    for (key, value) in params.into_iter().collect::<Vec<(&str, &str)>>() {
        context.insert(key, value);
    }

    tera.render(template, &context).expect("Failed to render template")
}

pub fn get_connection() -> SqliteConnection {
    let database: String = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "database/database.sqlite".to_string());

    SqliteConnection::establish(&database)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database))
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn get_from_form_body(field: String, req_body: String) -> String {
    req_body.split('&')
        .find(|param| param.starts_with(&format!("{}=", field)))
        .and_then(|param| param.split('=').nth(1))
        .map(|field_found| {
            urlencoding::decode(field_found)
                .map(|s| s.into_owned())
                .unwrap_or_else(|_| format!("Invalid {}", field))
        })
        .unwrap_or_else(|| format!("No {} provided", field))
}

pub fn is_authenticated(session: &Session) -> bool {
    let user_id = session.get::<i32>("user_id").unwrap_or(Some(0)).unwrap_or(0);

    user_id != 0
}

pub fn get_session_message(session: &Session, is_json: bool) -> (String, String) {
    if !is_json {
        let simple_message = ("success".to_string(), session
            .get::<String>("message")
            .unwrap_or(Some("".to_string()))
            .unwrap_or("".to_string()));

        session.remove("message");

        return simple_message;
    }

    let session_message = session
        .get::<serde_json::Value>("message")
        .unwrap_or(Some(serde_json::json!({})))
        .unwrap_or(serde_json::json!({}));

    if session_message.is_null() {
        session.remove("message");

        return ("success".to_string(), "".to_string());
    }

    if session_message["error"].is_string() && !session_message["error"].as_str().unwrap().is_empty() {
        session.remove("message");

        return ("error".to_string(), session_message["error"].as_str().unwrap().to_string());
    }

    if session_message["success"].is_string() && !session_message["success"].as_str().unwrap().is_empty() {
        session.remove("message");

        return ("error".to_string(), session_message["error"].as_str().unwrap().to_string());
    }

    session.remove("message");

    ("success".to_string(), "".to_string())
}
