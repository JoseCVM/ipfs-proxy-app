use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Request {
    pub id: i32,
    pub api_key: String,
    pub api_call: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "requests"]
pub struct NewRequest<'a> {
    pub api_key: &'a str,
    pub api_call: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
