// @generated automatically by Diesel CLI.

diesel::table! {
    customer_account (id) {
        id -> Int8,
        phone_number -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    customer_profile (customer_id) {
        customer_id -> Int8,
        nick_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    handyman_account (id) {
        id -> Int8,
        phone_number -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    handyman_expertise (id) {
        id -> Int8,
        handyman_id -> Int8,
        service -> entity_type::ServiceLayer2Mapping,
        note -> Nullable<Text>,
        rate_vnd -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    handyman_profile (handyman_id) {
        handyman_id -> Int8,
        first_name -> Text,
        last_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(customer_profile -> customer_account (customer_id));
diesel::joinable!(handyman_profile -> handyman_account (handyman_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer_account,
    customer_profile,
    handyman_account,
    handyman_expertise,
    handyman_profile,
);
