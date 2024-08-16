use diesel::prelude::*;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::users::dsl::users;
#[derive(Queryable, Selectable,Serialize,Deserialize)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub id:i32,
    pub username:String,
    pub password:String,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewUser {
    pub username: String,
    pub password: String,
}