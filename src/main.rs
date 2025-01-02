use std::io::{BufRead, Error};
use std::path::{PathBuf};
use include_dir::{include_dir, Dir};
use crate::procedures::project_structure::create_project_assets;

mod procedures {
    pub mod project_structure;
}

fn main() -> Result<(), Error> {
    static STUBS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/stubs");

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
    create_project_assets(
        STUBS_DIR,
        current_dir,
        PathBuf::from(destination)
    ).expect("Project creation failed");

    println!("Project created successfully!");

    Ok(())
}
