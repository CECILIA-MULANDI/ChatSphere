use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::{env, sync::Mutex};

// Define DbPool as a Mutex wrapping PgConnection
pub type DbPool = Mutex<PgConnection>;

// Establish a connection and wrap it in a Mutex
pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("The DATABASE_URL environment variable is not set");
    let conn = PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    Mutex::new(conn)  // Wrap PgConnection in Mutex
}
