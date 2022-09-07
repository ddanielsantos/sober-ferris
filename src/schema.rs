// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "geometry"))]
    pub struct Geometry;
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use super::sql_types::Geometry;

    partner (id) {
        id -> Varchar,
        trading_name -> Varchar,
        owner_name -> Varchar,
        document -> Varchar,
        address -> Geometry,
    }
}
