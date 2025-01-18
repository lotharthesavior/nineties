use std::env;
use diesel::{Connection, SqliteConnection};

pub fn get_connection() -> SqliteConnection {
    let database: String = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "database/database.sqlite".to_string());

    SqliteConnection::establish(&database)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database))
}
