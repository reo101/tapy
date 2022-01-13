use std::path::PathBuf;

use directories_next::ProjectDirs;

fn data_path() -> Option<PathBuf> {
    let path = ProjectDirs::from("bg", "reo101", "Tapy")?.data_dir().to_path_buf();
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    Some(path)
}

pub fn tags_path() -> Option<PathBuf> {
    let mut path = data_path()?;
    path.push("tags.json");
    Some(path)
}

pub fn media_path() -> Option<PathBuf> {
    let mut path = data_path()?;
    path.push("media");
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    Some(path)
}
