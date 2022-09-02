use diesel::prelude::*;
use dotenvy::dotenv;
use r2d2::{Pool, PooledConnection};
use diesel::r2d2::ConnectionManager;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbPoolConn = PooledConnection<ConnectionManager<SqliteConnection>>;

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

    let conn: &mut SqliteConnection = &mut pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS).expect("Couldn't run migrations");

    pool
}
