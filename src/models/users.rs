use chrono::NaiveDateTime;
use diesel::prelude::*;

use serde::{Deserialize,Serialize};


// use crate::schema::users::dsl::users;
#[derive(Queryable,Selectable,Serialize,Deserialize )]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub id:i32,
    pub username:String,
    pub password:String,
    pub created_at:chrono::NaiveDateTime,
    pub updated_at:chrono::NaiveDateTime
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewUser {
    pub username: String,
    pub password: String,
}