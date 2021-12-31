use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub struct User {
    id: i32,
    pub uuid: uuid::Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    password: String,
    token: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable, Identifiable, Deserialize)]
#[table_name = "users"]
pub struct FormUser<'a> {
    id: i32,
    pub uuid: uuid::Uuid,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    password: String,
    token: String,
    pub enabled: bool
}