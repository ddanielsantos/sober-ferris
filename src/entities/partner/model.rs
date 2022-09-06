use diesel::prelude::*;
use diesel::Queryable;
use rocket::serde::Serialize;
use crate::schema::partner::dsl;

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Partner {
    pub id: String,
    pub tradingname: String,
    pub ownername: String,
    pub document: String
}

impl Partner {
    pub fn find_one(id: &str, conn: &mut PgConnection) -> Partner {
        let res = dsl::partner
            .find(id)
            .first::<Partner>(conn)
            .unwrap();

        res
    }
}
