use diesel::{SqliteConnection};

pub trait Seeder {
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>>;
}
