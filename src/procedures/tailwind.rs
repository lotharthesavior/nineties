use std::io::{read_to_string, BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn verify_dependencies(current_dir: PathBuf) -> bool {
    let command: &str = r#"
        if ! command -v npm &> /dev/null; then
          exit 1
        fi
    "#;

    let output = Command::new("bash")
        .current_dir(current_dir)
        .args(["-c", command])
        .output()
        .expect("Failed to spawn command");

    output.status.success()
}

pub fn install_tailwind(current_dir: PathBuf) -> () {
    let command : &str = include_str!("../../scripts/install_tailwind.sh");

    let mut child = Command::new("/bin/bash")
        .current_dir(current_dir)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line), // Stream the output line by line
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }

    let status = child.wait().expect("Failed to wait on tailwind installation");

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}
