use diesel::{Insertable, SqliteConnection};
use crate::models::user::{NewUser};
use crate::schema::users::dsl::users;

pub struct UserSeeder;

pub trait Seeder {
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>>;
}

impl Seeder for UserSeeder {
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
        println!("Creating users table");

        let _ = diesel::insert_into(users).values(NewUser {
            name: "Jekyll",
            email: "jekyll@example.com",
            password: "password",
        });

        Ok(())
    }
}