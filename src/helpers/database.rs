use std::env;
use diesel::{SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub fn get_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    let pool = get_connection_pool();
    let pool = pool.clone();

    pool.get().unwrap()
}

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database: String = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "database/database.sqlite".to_string());
    let pool_limit: u32 = env::var("DATABASE_POOL_LIMIT")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("DATABASE_POOL_LIMIT must be a number");

    let manager = ConnectionManager::<SqliteConnection>::new(database);

    Pool::builder()
        .max_size(pool_limit)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}