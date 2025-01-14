use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub addres: String,
    pub created_at: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserNew<'a>{
    pub name: &'a str,
    pub address: &'a str,
    pub created_at: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub name: String,
    pub address: String,
}