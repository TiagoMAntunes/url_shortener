use derive_more::Display;

#[derive(Display)]
pub enum DatabaseError {
    NotFound,
}
