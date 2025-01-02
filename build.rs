use std::{env, fs, path::Path};

fn main() {
    // Get the output directory provided by Cargo
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let stubs_src = Path::new("stubs"); // Path to your stubs directory
    let stubs_dest = Path::new(&out_dir).join("stubs"); // Destination in OUT_DIR

    // Check if the source directory exists
    if !stubs_src.exists() {
        panic!("The 'stubs' directory does not exist at {:?}", stubs_src);
    }

    // Recursively copy the stubs directory to the destination
    copy_dir(stubs_src, &stubs_dest).expect("Failed to copy 'stubs' directory");

    // Inform Cargo to rerun the build script if the stubs directory changes
    println!("cargo:rerun-if-changed=stubs");
}

// Helper function to copy directories recursively
fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}
