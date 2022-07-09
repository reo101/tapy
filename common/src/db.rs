use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = crate::util::fs::database_path()
        .expect("DATABASE_URL must be set")
        .into_os_string()
        .into_string()
        .expect("Bruv, replace with unwrap()");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
