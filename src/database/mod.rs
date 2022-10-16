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

        let mut conn = match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(DatabaseError::Connection),
        }?;

        let result = match urls
            .filter(shortened.eq(url))
            .first::<models::URLPair>(&mut conn)
        {
            Ok(res) => Ok(res),
            Err(diesel::NotFound) => Err(DatabaseError::NotFound),
            Err(_) => Err(DatabaseError::Internal),
        }?;

        Ok(result.original)
    }
    pub async fn save_url(&self, url: &str) -> Result<String, DatabaseError> {
        for i in 0..5 {
            let mut conn = match self.pool.get() {
                Ok(conn) => Ok(conn),
                Err(_) => Err(DatabaseError::Connection),
            }?;

            let res = conn.transaction::<String, diesel::result::Error, _>(|conn| {
                use schema::urls::dsl::*;

                if let Ok(pair) = urls.filter(original.eq(url)).first::<models::URLPair>(conn) {
                    // URL has already been shortened
                    return Ok(pair.shortened);
                }

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
                    .execute(conn)?;

                log::debug!("Inserted to database in {} tries", i + 1);
                Ok(shortened_url)
            });

            let url = match res {
                Ok(url) => url,
                Err(_) => continue, // Attempt to insert again
            };

            return Ok(url);
        }

        Err(DatabaseError::UnableToInsert)
    }
}
