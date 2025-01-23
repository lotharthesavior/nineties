use actix_session::Session;
use diesel::{QueryDsl, RunQueryDsl};
use crate::helpers::database::get_connection;
use crate::models::user::User;
use crate::schema::users::dsl::users;

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

pub fn get_session_user(session: &Session) -> Option<User> {
    let user_id: i32 = session.get::<i32>("user_id").unwrap_or(Some(0)).unwrap_or(0);
    let user: Result<User, diesel::result::Error> = users.find(user_id).first::<User>(&mut get_connection());

    if user.is_ok() {
        Some(user.unwrap())
    } else {
        None
    }
}
