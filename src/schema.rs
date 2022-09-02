// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "geometry"))]
    pub struct Geometry;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Geometry;

    partner (id) {
        id -> Varchar,
        tradingname -> Varchar,
        ownername -> Varchar,
        document -> Varchar,
        coveragearea -> Geometry,
        address -> Geometry,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    partner,
    spatial_ref_sys,
);
