// @generated automatically by Diesel CLI.

diesel::table! {
    partner (id) {
        id -> Varchar,
        trading_name -> Varchar,
        owner_name -> Varchar,
        document -> Varchar,
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
