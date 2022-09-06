use super::model::Partner;
use rocket::serde::json::Json;
use crate::establish_connection;

#[get("/<id>")]
pub fn partner_by_id (id: &str) -> Json<Partner> {
    let connection = &mut establish_connection();
    let result = Partner::find_one(id, connection);

    Json(result)
}
