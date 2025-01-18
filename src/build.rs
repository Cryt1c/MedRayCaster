use std::fs::{self};

fn get_cargo_target_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let profile = std::env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}

fn copy_dir_recursive(source: &str, destination: &str) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = format!("{}/{}", destination, entry.file_name().to_string_lossy());

        if path.is_dir() {
            copy_dir_recursive(path.to_str().unwrap(), &dest_path)?;
        } else {
            fs::copy(path, dest_path)?;
        }
    }
    Ok(())
}

fn main() {
    let cargo_target_dir = get_cargo_target_dir().expect("cargo_target_dir could not be found");
    fs::create_dir_all(cargo_target_dir.join("assets/shaders"))
        .expect("assets/shaders could not be created");
    copy_dir_recursive(
        "assets/shaders",
        cargo_target_dir
            .join("assets/shaders")
            .into_os_string()
            .to_str()
            .expect("Could not create str from os_string 'assets/shaders'"),
    )
    .expect("Could not copy shaders");

    fs::create_dir_all(cargo_target_dir.join("assets/data"))
        .expect("assets/data could not be created");
    copy_dir_recursive(
        "assets/data",
        cargo_target_dir
            .join("assets/data")
            .into_os_string()
            .to_str()
            .expect("Could not create str from os_string 'assets/data'"),
    )
    .expect("Could not copy data");
}
