use std::fs;
use std::io::Error;
use std::path::PathBuf;
use include_dir::{Dir};

fn recursive_asset_copy(stubs_dir: Dir, target_path: PathBuf, original_base_dir: PathBuf) {
    for file in stubs_dir.files() {
        let target_path1 = target_path.clone().join(file.path());
        fs::write(target_path1, file.contents()).expect("Failed to write file");
    }

    for dir in stubs_dir.dirs() {
        let target_path2 = target_path.clone().join(dir.path());
        fs::create_dir_all(&target_path2).expect("Failed to create directories");
        recursive_asset_copy(*dir, target_path.clone(), original_base_dir.clone());
    }
}

pub fn create_project_assets(stubs_dir: Dir, base_dir: PathBuf, target_path: PathBuf) -> Result<(), Error> {
    println!("Creating project assets in {}", target_path.as_path().to_str().unwrap_or(""));

    if !fs::exists(target_path.clone()).unwrap() {
        fs::create_dir_all(target_path.clone());
    }

    recursive_asset_copy(stubs_dir, target_path, base_dir);
    Ok(())
}
