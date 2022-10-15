#[derive(Clone)]
pub struct Database {}


pub enum DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Database {
    pub fn new() -> Database {
        Database {}
    }
    pub async fn fetch_url(&self, url: &str) -> Result<String, DatabaseError> {
        Ok(format!(""))
    }
    pub async fn save_url(&self, url: &str) -> Result<String, DatabaseError> {
        Ok(format!(""))
    }
}
