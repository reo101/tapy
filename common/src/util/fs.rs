use std::path::PathBuf;

use directories_next::ProjectDirs;
use dotenv::dotenv;

fn data_path() -> Option<PathBuf> {
    let path = ProjectDirs::from("bg", "reo101", "Tapy")?.data_dir().to_path_buf();
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    Some(path)
}

pub fn database_path() -> Option<PathBuf> {
    dotenv().ok();

    match std::env::var("DATABASE_URL") {
        Result::Ok(url) => Some(PathBuf::from(url)),
        Result::Err(_) => {
            let mut path = data_path()?;
            path.push("db.db");
            Some(path)
        }
    }
}

pub fn media_path() -> Option<PathBuf> {
    let mut path = data_path()?;
    path.push("media");
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    Some(path)
}
