use super::schema;

// This file keeps track of all the models to access the database
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = schema::urls)]
pub struct URLPairInsert<'a> {
    pub shortened: &'a str,
    pub original: &'a str,
}

#[derive(Queryable)]
pub struct URLPair {
    pub shortened : String,
    pub original : String,
}