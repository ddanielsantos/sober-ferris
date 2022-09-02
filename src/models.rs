use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Partner {
    pub id: String,
    pub tradingname: String,
    pub ownername: String,
    pub document: String
}
