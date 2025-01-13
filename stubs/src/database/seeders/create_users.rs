use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::{Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use crate::database::seeders::traits::seeder::Seeder;
use crate::models::user::NewUser;
use crate::schema::users::dsl::*;

pub struct UserSeeder;

impl Seeder for UserSeeder {
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
        let expected_email: &str = "jekyll@example.com";

        let all_users: Vec<i32> = users.select(id).load::<i32>(conn).unwrap();
        if all_users.len() > 0 {
            println!("User table already seeded: {}", expected_email);
            return Ok(());
        }

        let password_bytes = b"password";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password_bytes, &salt).unwrap().to_string();

        println!("Creating users...");
        let _ = diesel::insert_into(users).values(NewUser {
            name: "Jekyll",
            email: expected_email,
            password: &*password_hash,
        }).execute(conn);

        Ok(())
    }
}