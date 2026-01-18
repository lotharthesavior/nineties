use std::env;

#[cfg(test)]
use crate::helpers::database::reset_pool;

#[allow(dead_code)]
pub struct TestFinalizer;
impl Drop for TestFinalizer {
    fn drop(&mut self) {
        // Reset the connection pool first to release all connections
        #[cfg(test)]
        reset_pool();

        let database: String = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "database/database.sqlite".to_string());
        let _ = std::fs::remove_file(database);
    }
}
