use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use tera::{Context, Tera};

pub fn create_project_assets(current_dir: PathBuf) -> () {
    let _ = fs::create_dir_all(current_dir.join("dist"));
    let _ = fs::create_dir_all(current_dir.join("src/controllers"));
    let _ = fs::create_dir_all(current_dir.join("src/resources"));
    let _ = fs::create_dir_all(current_dir.join("src/resources/css"));
    let _ = fs::create_dir_all(current_dir.join("src/resources/js"));
    let _ = fs::create_dir_all(current_dir.join("src/resources/views"));

    // Step 3: add stubs

    let tera = match Tera::new("stubs/**/*.stub") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/controllers/controller.rs.stub"),
        &"src/controllers/home_controller.rs",
        &vec![("controller_name", "home")]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/resources/css/styles.css.stub"),
        &"src/resources/css/styles.css",
        &vec![]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/resources/js/script.js.stub"),
        &"src/resources/js/script.js",
        &vec![("project_name", "Nineties")]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/resources/views/home.html.stub"),
        &"src/resources/views/home.html",
        &vec![("name", "Nineties"), ("project_name", "Nineties")]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/main.rs.stub"),
        &"src/main.rs",
        &vec![("host", "127.0.0.1"), ("port", "8080")]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/routes.rs.stub"),
        &"src/routes.rs",
        &vec![]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/helpers.rs.stub"),
        &"src/helpers.rs",
        &vec![]
    );

    add_file_to(
        tera.clone(),
        current_dir.clone(),
        include_str!("../../stubs/root/package.json.stub"),
        &"package.json",
        &vec![]
    );
}

fn add_file_to(
    mut tera: Tera,
    current_dir: PathBuf,
    stub: &str,
    file_name: &str,
    params: &Vec<(&str, &str)>
) -> () {
    let mut context = Context::new();

    for (key, value) in params.into_iter() {
        context.insert(key.to_string(), value);
    }

    let stub_content = stub;
    let rendered = match tera.render_str(stub_content, &context) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to render template (file {}): {}", file_name, e);
            return;
        }
    };

    let _ = fs::remove_file(current_dir.join(file_name));
    if let Err(e) = fs::write(current_dir.join(file_name), rendered.to_string()) {
        eprintln!("Failed to write file: {}", e);
    }
}

pub fn run_cargo_add_dependencies() {
    let dependencies = vec![
        "actix-files@0.6.6",
        "actix-web@4",
        "dotenv@0.15.0",
        "tera@1.20.0",
    ];

    for dep in dependencies {
        let output = Command::new("cargo")
            .arg("add")
            .arg(dep)
            .output()
            .expect("Failed to execute cargo add command");

        if !output.status.success() {
            eprintln!(
                "Failed to add dependency {}: {}",
                dep,
                String::from_utf8_lossy(&output.stderr)
            );
        } else {
            println!("Successfully added dependency {}", dep);
        }
    }
}
