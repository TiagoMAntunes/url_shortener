pub mod errors;
mod setup;

use diesel::{r2d2, PgConnection};

pub use errors::DatabaseError;
// pub use setup::MIGRATIONS as SCHEMA;
pub use setup::run_migrations;

#[derive(Clone)]
pub struct Database {
    pool: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            pool: setup::create_db_pool(),
        }
    }

    pub fn setup_database(&self) {
        let mut conn = self
            .pool
            .get()
            .expect("Failed to obtain a connection from the pool");
        run_migrations(&mut conn);
    }

    pub async fn fetch_url(&self, url: &str) -> Result<String, DatabaseError> {
        Ok(format!(""))
    }
    pub async fn save_url(&self, url: &str) -> Result<String, DatabaseError> {
        Ok(format!(""))
    }
}
