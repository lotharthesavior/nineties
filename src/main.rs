use std::io::{BufRead, Error};
use std::path::PathBuf;
use crate::procedures::{project_structure, tailwind};

mod procedures {
    pub mod tailwind;
    pub mod project_structure;
}

fn main() -> Result<(), Error> {

    // The following code will be executed in the current command line path

    let current_dir: PathBuf = std::env::current_dir().expect("Failed to get current directory");

    if !tailwind::verify_dependencies(current_dir.clone()) {
        eprintln!("npm is not installed. Please install it and try again.");
        std::process::exit(1);
    }

    let current_dir_tw_clone = current_dir.clone();
    let handle_tw = std::thread::spawn(|| {
        tailwind::install_tailwind(current_dir_tw_clone);
    });

    let current_dir_project_clone = current_dir.clone();
    let handle_project = std::thread::spawn(|| {
        project_structure::create_project_assets(current_dir_project_clone);
    });

    project_structure::run_cargo_add_dependencies();


    handle_tw.join().expect("Thread panicked");
    handle_project.join().expect("Thread panicked");

    println!("Project created successfully!");

    Ok(())
}

// TODO: move this to a command called "nineties build"
// TODO: command "nineties dev":
//       - thread 1: cargo run
//       - thread 2: npx tailwindcss -i ./src/input.css -o ./src/output.css --watch
