use std::fs;
use std::io::Error;
use std::path::PathBuf;
use include_dir::{include_dir, Dir};

fn recursive_asset_copy(stubs_dir: Dir, target_path: PathBuf, original_base_dir: PathBuf) {
    println!("#########################################################");
    println!("#########################################################");
    println!("Creating project assets in {:?}", target_path.clone());

    for file in stubs_dir.files() {
        let target_path1 = target_path.clone().join(file.path());
        println!("Creating file {:?} ({:?})", target_path1, file.path());
        fs::write(target_path1, file.contents()).expect("Failed to write file");
    }

    for dir in stubs_dir.dirs() {
        let target_path2 = target_path.clone().join(dir.path());
        println!("Creating directory {:?}", target_path2);

        fs::create_dir_all(&target_path2).expect("Failed to create directories");
        recursive_asset_copy(*dir, target_path.clone(), original_base_dir.clone());
    }
}

pub fn create_project_assets(base_dir: PathBuf, target_path: PathBuf) -> Result<(), Error> {
    static STUBS_DIR: Dir = include_dir!("stubs");
    println!("Creating project assets in {:?}", STUBS_DIR);

    recursive_asset_copy(STUBS_DIR, target_path, base_dir);

    Ok(())
}
