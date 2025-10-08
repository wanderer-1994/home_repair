// @generated automatically by Diesel CLI.

diesel::table! {
    handyman (handyman_id) {
        handyman_id -> Int8,
        full_name -> Nullable<Text>,
        skills -> Nullable<Array<Nullable<entity_type::ServiceLayer2Mapping>>>,
        search_vector -> Nullable<diesel_full_text_search::TsVector>,
    }
}
