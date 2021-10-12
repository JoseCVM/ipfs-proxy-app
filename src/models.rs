use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Request {
    pub id: i32,
    pub api_key: String,
    pub api_call: String,
    pub request_size: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "requests"]
pub struct NewRequest<'a> {
    pub api_key: &'a str,
    pub api_call: &'a str,
    pub request_size: i32,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Key {
    pub api_key: String,
    pub expires_in: i32,
    pub is_enabled: bool,
    pub userid: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "keys"]
pub struct NewKey<'a> {    
    pub api_key: &'a str,
    pub expires_in: i32,
    pub is_enabled: bool,
    pub userid: i32,
    pub created_at: chrono::NaiveDateTime,
}
