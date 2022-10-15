pub mod errors;
mod models;
mod schema;
mod setup;

use diesel::{r2d2, PgConnection};
use diesel::{QueryDsl, RunQueryDsl};

use diesel::prelude::*;
pub use errors::DatabaseError;
use rand::distributions::Alphanumeric;

use rand::Rng;
pub use setup::run_migrations;
pub use setup::MIGRATIONS as SCHEMA;

use crate::database::models::URLPair;

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
        use schema::urls::dsl::*;

        let result = urls
            .filter(shortened.eq(url))
            .first::<models::URLPair>(&mut self.pool.get().unwrap())
            .unwrap();

        Ok(result.original)
    }
    pub async fn save_url(&self, url: &str) -> Result<String, DatabaseError> {
        // Generate random string of 10 characters
        let shortened_url = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        let payload = models::URLPairInsert {
            shortened: &shortened_url,
            original: url,
        };

        use schema::urls;

        diesel::insert_into(urls::table)
            .values(&payload)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error saving url");

        Ok(shortened_url)
    }
}
