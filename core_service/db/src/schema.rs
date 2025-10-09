// @generated automatically by Diesel CLI.

diesel::table! {
    handyman_service (id) {
        id -> Int8,
        handyman_id -> Int8,
        service -> entity_type::ServiceLayer2Mapping,
        note -> Nullable<Text>,
        rate_vnd -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}
