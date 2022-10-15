use diesel::{r2d2, Connection, PgConnection};


// Run migrations at compile time
pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("./migrations");

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
