use diesel::prelude::*;
use super::model::Partner;
use rocket::serde::json::Json;
use crate::establish_connection;
use crate::schema::partner::dsl;

#[get("/<id>")]
pub fn partner_by_id (id: &str) -> Json<Partner> {
    let connection = &mut establish_connection();
    let result = dsl::partner
        .filter(dsl::id.eq(id))
        .first::<Partner>(connection);

    let partners = result.unwrap();

    Json(partners)
}
