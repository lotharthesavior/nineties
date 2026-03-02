use diesel::SqliteConnection;
use std::error::Error;

/// Trait for database seeders that populate tables with initial data.
pub trait Seeder {
    /// Executes the seeder against the given database connection.
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>>;
}
