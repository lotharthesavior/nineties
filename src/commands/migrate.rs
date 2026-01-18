use crate::database::seeders::create_users::UserSeeder;
use crate::database::seeders::traits::seeder::Seeder;
use crate::helpers::database::get_connection;
use crate::models::user::MIGRATIONS;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use diesel_migrations::MigrationHarness;
use std::env;
use std::fs;
use std::io;

pub fn run(args: &[String]) -> io::Result<()> {
    println!("Starting migration procedure.");

    if args.contains(&"--fresh".to_string()) {
        println!("Reverting all migrations...");
        let database: String =
            env::var("DATABASE_URL").unwrap_or_else(|_| "database/database.sqlite".to_string());
        fs::remove_file(database).expect("Failed to remove database file");
    }

    println!("Running migrations...");
    let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = get_connection();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    if args.contains(&"--seed".to_string()) {
        println!("Running seeders...");
        UserSeeder::execute(&mut get_connection()).expect("Failed to seed users table");
    }

    Ok(())
}
