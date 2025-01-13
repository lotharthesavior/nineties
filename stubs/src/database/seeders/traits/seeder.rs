use diesel::SqliteConnection;
use std::error::Error;

pub trait Seeder {
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>>;
}
