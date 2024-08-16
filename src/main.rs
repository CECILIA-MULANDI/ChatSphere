mod db_connections;
mod controllers;
mod models;
mod schema;
use controllers::users::{register_user,render_register_form};
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use db_connections::db::establish_connection;
use dotenvy::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;

async fn home_page()->impl Responder{
    HttpResponse::Ok().body("home page here")
}
#[actix_web::main]
async fn main()->std::io::Result<()> {
    // load dotenv files
    dotenv().ok();
    let host = env::var("HOST").expect("The host was not found");
    let port=env::var("PORT").expect("The port cannot be accessed");
    let server=format!("{}:{}",host,port);
    let pool=establish_connection();
    let db_conn=web::Data::new(pool);
    HttpServer::new(move||{
        App::new()
        .app_data(db_conn.clone())
        .route("/register", web::get().to(render_register_form)) 
        .route("/register", web::post().to(register_user))
        
    
    }).bind(server  )?
    .run()
    .await
}
