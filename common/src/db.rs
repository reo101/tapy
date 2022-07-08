use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
    //     crate::util::fs::database_path()
    //         .expect("DATABASE_URL must be set")
    //         .into_os_string()
    //         .into_string()
    //         .expect("Bruv, eplace with unwrap()")
    // });

    let database_url = crate::util::fs::database_path()
        .expect("DATABASE_URL must be set")
        .into_os_string()
        .into_string()
        .expect("Bruv, eplace with unwrap()");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
