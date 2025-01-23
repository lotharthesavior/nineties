use diesel::{Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use crate::database::seeders::traits::seeder::Seeder;
use crate::models::user::NewUser;
use crate::schema::users::dsl::*;
use crate::services::user_service::prepare_password;

pub struct UserSeeder;

impl Seeder for UserSeeder {
    fn execute(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
        let expected_email: &str = "jekyll@example.com";

        let all_users: Vec<i32> = users.select(id).load::<i32>(conn).unwrap();
        if all_users.len() > 0 {
            println!("User table already seeded: {}", expected_email);
            return Ok(());
        }

        println!("Creating users...");
        let _ = diesel::insert_into(users).values(NewUser {
            name: "Jekyll",
            email: expected_email,
            password: &*prepare_password("password"),
        }).execute(conn);

        Ok(())
    }
}