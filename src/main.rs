use dotenv::dotenv;
use std::env;
use std::io::{BufRead, Error};
use std::path::{PathBuf};
use include_dir::{include_dir, Dir};
use crate::procedures::project_structure::create_project_assets;

mod procedures {
    pub mod project_structure;
}

fn main() -> Result<(), Error> {
    dotenv().ok();

    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let environment: String = env::var("ENVIRONMENT").unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a project name");
        return Ok(());
    }
    let destination_dir_string = &args[1];

    if destination_dir_string.contains("/") {
        println!("Please provide a valid project name, this is not valid due to slashes: {}", destination_dir_string);
        return Ok(());
    }
    let destination = current_dir.join(&args[1]);

    let mut stub_dir: Dir;
    if environment == "debian" {
        stub_dir = include_dir!("/var/www/Agency/nineties/stubs");
    } else {
        stub_dir = include_dir!("stubs");
    }

    println!("Creating project {}...", args[1]);
    create_project_assets(
        stub_dir,
        current_dir,
        PathBuf::from(destination)
    ).expect("Project creation failed");

    println!("Project created successfully!");

    Ok(())
}
