use std::io::{self, ErrorKind, Error};
use std::path::PathBuf;

use directories_next::ProjectDirs;

pub fn media_iter() -> io::Result<impl Iterator<Item = PathBuf>> {
    macro_rules! string_vec {
        {
            $( $elem: expr ),* $(,)?
        } => {{
            let mut vec: Vec<String> = vec![];
            $( vec.push(String::from($elem)); )*
            vec
        }}
    }

    // TODO: Load from config
    let allowed_extensions = string_vec!["gif"];

    if let Some(proj_dirs) = ProjectDirs::from("bg", "reo101", "Tapy") {
        return Ok(proj_dirs
            .data_dir()
            .read_dir()?
            .map(|res| res.map(|e| e.path()))
            .filter_map(move |res| {
                let path = res.ok()?;
                match path.extension()?.to_str()? {
                    ext if allowed_extensions.contains(&String::from(ext)) => Some(path),
                    _ => None,
                }
            }));
    }

    Err(Error::new(ErrorKind::NotFound, "Project data directory could not be retrieved"))
}
