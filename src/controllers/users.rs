
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST};

use crate::db_connections::db::{self, DbPool};
use crate::models::users::{self, NewUser};
use serde::Deserialize;
use crate::schema::users::table;
#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

// Define the RegisterTemplate struct for rendering the HTML form
#[derive(Template)]
#[template(path = "../templates/register.html")]
pub struct RegisterTemplate;

// Handler to render the registration form
pub async fn render_register_form() -> impl Responder {
    let template = RegisterTemplate;
    HttpResponse::Ok().body(template.render().unwrap())
}

// Handler to register a new user
pub async fn register_user(
    form: web::Json<RegisterForm>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let conn = pool.lock().expect("Failed to connect to the database");
    
    // Hash the password
    let hashed_password = hash(&form.password, DEFAULT_COST).expect("Failed to hash the password");
    
    // Create an instance of a new user
    let new_user = NewUser {
        username: form.username.clone(),
         password: hashed_password,
    };
    
    // Insert new user into the database
    let result = diesel::insert_into(table)
        .values(&new_user)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().json("User has been registered successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error registering user."),
    }
}
