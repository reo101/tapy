use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = crate::util::fs::database_path()
        .map(std::path::PathBuf::into_os_string)
        .map(std::ffi::OsString::into_string)
        .and_then(std::result::Result::ok)
        .expect("Error setting database path");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
