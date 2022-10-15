#[derive(Clone)]
pub struct Database {}

pub enum DatabaseError {}

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
