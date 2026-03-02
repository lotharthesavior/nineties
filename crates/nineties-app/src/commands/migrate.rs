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
use tracing::info;

/// Runs pending database migrations. Supports `--fresh` to drop and recreate
/// the database, and `--seed` to populate with default data after migration.
pub fn run(args: &[String]) -> io::Result<()> {
    info!("Starting migration procedure");

    if args.contains(&"--fresh".to_string()) {
        info!("Reverting all migrations");
        let database: String =
            env::var("DATABASE_URL").unwrap_or_else(|_| "database/database.sqlite".to_string());
        fs::remove_file(&database).expect("Failed to remove database file");
        info!("Removed database file: {}", database);
    }

    info!("Running pending migrations");
    let mut conn: PooledConnection<ConnectionManager<SqliteConnection>> = get_connection();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    info!("Migrations completed successfully");

    if args.contains(&"--seed".to_string()) {
        info!("Running seeders");
        UserSeeder::execute(&mut get_connection()).expect("Failed to seed users table");
        info!("Seeders completed successfully");
    }

    Ok(())
}
