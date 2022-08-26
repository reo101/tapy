use diesel::prelude::*;
use dotenvy::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// TODO: conditionally compile the apropriate database provider

pub fn establish_connection() -> DbPool {
    if cfg!(test) {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Couldn't create Database Pool");

        let conn: &SqliteConnection = &pool.get().unwrap();
        diesel_migrations::run_pending_migrations(conn).expect("Couldn't run migrations for tests");

        pool
    } else {
        dotenv().ok();

        let database_url = crate::db::util::fs::database_path()
            .map(std::path::PathBuf::into_os_string)
            .map(std::ffi::OsString::into_string)
            .and_then(std::result::Result::ok)
            .expect("Error setting database path");

        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("Couldn't create Database Pool")
    }
}
