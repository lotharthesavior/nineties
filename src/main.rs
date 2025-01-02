use std::io::{BufRead, Error};
use std::path::PathBuf;
use crate::procedures::{project_structure, tailwind};
use crate::procedures::project_structure::create_project_assets;

mod procedures {
    pub mod tailwind;
    pub mod project_structure;
}

fn main() -> Result<(), Error> {
    let current_dir: PathBuf = std::env::current_dir().expect("Failed to get current directory");
    let args: Vec<String> = std::env::args().collect();

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

    println!("Creating project {}...", args[1]);
    create_project_assets(current_dir, PathBuf::from(destination)).expect("Project creation failed");

    println!("Project created successfully!");

    Ok(())
}
