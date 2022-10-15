use crate::diesel_migrations::MigrationHarness;
use diesel::{pg::PgConnection, r2d2};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

// Generate migrations at compile time
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_db_pool() -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    let db_url = format!(
        "postgresql://{}:{}@postgres:5432",
        std::env::var("DB_USER").expect("DB_USER environment variable missing"),
        std::env::var("DB_PASS").expect("DB_PASS environment variable missing"),
    );

    r2d2::Pool::builder()
        .build(r2d2::ConnectionManager::new(&db_url))
        .expect("Failed to create pool.")
}

// Run migrations on startup in production
pub fn run_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
