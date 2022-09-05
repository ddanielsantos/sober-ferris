use diesel::prelude::*;
use rocket::serde::json::Json;
use crate::establish_connection;
use crate::models::partner::Partner;
use crate::schema::partner::dsl as partner_dsl;

#[get("/<id>")]
pub fn partner_by_id (id: &str) -> Json<Partner> {
    let connection = &mut establish_connection();
    let result = partner_dsl::partner
        .filter(partner_dsl::id.eq(id))
        .first::<Partner>(connection);

    let partners = result.unwrap();

    Json(partners)
}
