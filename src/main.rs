use std::env;
use self::models::*;
use diesel::prelude::*;
use rocket::serde::json::Json;
use diesel::{PgConnection, Connection};
use dotenvy::dotenv;

pub mod models;
pub mod schema;

#[macro_use] extern crate rocket;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("You must set a DATABASE_URL environment variable");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/partner/<id>")]
fn partner_by_id (id: &str) -> Json<Partner> {
    use self::schema::partner::dsl as partner;
    let connection = &mut establish_connection();
    let result = partner::partner.filter(partner::id.eq(id)).first::<Partner>(connection);
    let partners = result.unwrap();

    Json(partners)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, partner_by_id])
}
