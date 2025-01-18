use std::{env, fs};
use std::collections::HashMap;
use std::io::Error;
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
use diesel::{Connection, SqliteConnection, QueryDsl, RunQueryDsl};
use crate::models::user::User;
use crate::schema::users::dsl::users;

pub fn load_template(template: &str, params: Vec<(&str, &str)>, assets: Option<Vec<&str>>) -> String {
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

    context.insert("assets", &get_assets_string(assets));

    tera.render(template, &context).expect("Failed to render template")
}

// Here we return the html string to add the assets to the template.
// If the assets are passed, we only add the assets passed, otherwise we add all the assets from
// the manifest.json file.
fn get_assets_string(assets: Option<Vec<&str>>) -> String {
    let mut assets_string: String = String::new();
    if assets.is_none() {
        for (key, value) in get_manifest_assets().into_iter().enumerate() {
            let asset_type = value.1.split('.').last().unwrap();
            if asset_type == "css" {
                assets_string.push_str(&format!("<link rel=\"stylesheet\" href=\"/public/{}\">", value.1));
            } else if asset_type == "js" {
                assets_string.push_str(&format!("<script src=\"/public/{}\" defer></script>", value.1));
            }
        }
    } else {
        let manifest_assets = get_manifest_assets();
        for (key, value) in assets.unwrap().into_iter().enumerate() {
            let asset_type = value.split('.').last().unwrap();
            let asset = manifest_assets.get(value);
            if asset_type == "css" && asset.is_some() {
                assets_string.push_str(&format!("<link rel=\"stylesheet\" href=\"/public/{}\">", asset.unwrap()));
            } else if asset_type == "js" && asset.is_some() {
                assets_string.push_str(&format!("<script src=\"/public/{}\" defer></script>", asset.unwrap()));
            }
        }
    }

    assets_string
}

fn get_tailwind_asset_string() -> String {
    let mut assets_string: String = String::new();
    assets_string.push_str("<link rel=\"stylesheet\" href=\"/styles.css\">");
    assets_string.push_str("<script src=\"/scripts.js\" defer></script>");

    assets_string
}

// Here we get the assets from the manifest.json file.
fn get_manifest_assets() -> HashMap<String, String> {
    let mut assets: HashMap<String, String> = HashMap::new();
    let manifest: Result<String, Error> = fs::read_to_string("dist/.vite/manifest.json");
    if manifest.is_ok() {
        let manifest: String = manifest.unwrap();
        let manifest_json: serde_json::Value = serde_json::from_str(&manifest).expect("Failed to parse manifest.json");

        for (key, value) in manifest_json.as_object().unwrap().iter() {
            let asset = value.get("file");
            if asset.is_some() {
                assets.insert(key.to_string(), asset.unwrap().as_str().unwrap().parse().unwrap());

                // If the asset is a js file, we might add css files to the assets.
                let asset_type = asset.unwrap().as_str().unwrap().split('.').last().unwrap();
                if asset_type == "js" {
                    for css_file in value.get("css").unwrap().as_array().unwrap() {
                        let css_file_name = css_file.as_str().unwrap().split('/').last().unwrap();
                        assets.insert(css_file_name.to_string(), css_file_name.to_string());
                    }
                }
            }
        }
    }

    assets
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
    let user_id: i32 = session.get::<i32>("user_id").unwrap_or(Some(0)).unwrap_or(0);
    let user: Result<User, diesel::result::Error> = users.find(user_id).first::<User>(&mut get_connection());

    user.is_ok()
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
