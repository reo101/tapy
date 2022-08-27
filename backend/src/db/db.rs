use diesel::prelude::*;
use dotenvy::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = if cfg!(test) {
        ":memory:".to_string()
    } else {
        dotenv().ok();

        crate::db::util::fs::database_path()
            .map(std::path::PathBuf::into_os_string)
            .map(std::ffi::OsString::into_string)
            .and_then(std::result::Result::ok)
            .expect("Error setting database path")
    };

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Couldn't create Database Pool");

    let conn: &SqliteConnection = &pool.get().unwrap();
    embedded_migrations::run(conn).expect("Couldn't run migrations");

    pool
}
