// @generated automatically by Diesel CLI.

diesel::table! {
    customer_account (id) {
        id -> Int8,
        phone_number -> Text,
        password_hash -> Text,
    }
}

diesel::table! {
    customer_profile (customer_account_id) {
        customer_account_id -> Int8,
        mick_name -> Text,
    }
}

diesel::table! {
    handyman_account (id) {
        id -> Int8,
        phone_number -> Text,
        password_hash -> Text,
    }
}

diesel::table! {
    handyman_profile (handyman_id) {
        handyman_id -> Int8,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::joinable!(customer_profile -> customer_account (customer_account_id));
diesel::joinable!(handyman_profile -> handyman_account (handyman_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer_account,
    customer_profile,
    handyman_account,
    handyman_profile,
);
