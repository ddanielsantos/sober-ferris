use std::env;
use dotenvy::dotenv;
use diesel::{PgConnection, Connection};
use crate::routes::partner::partner_by_id;

pub mod routes;
pub mod models;
pub mod schema;

#[macro_use] extern crate rocket;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("You must set a DATABASE_URL environment variable");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/partner", routes![partner_by_id])
}
