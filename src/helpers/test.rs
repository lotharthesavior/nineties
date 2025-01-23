use std::env;

pub struct TestFinalizer;
impl Drop for TestFinalizer {
    fn drop(&mut self) {
        let database: String = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "database/database.sqlite".to_string());
        let _ = std::fs::remove_file(database);
    }
}
