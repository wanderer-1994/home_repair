use diesel::{
    define_sql_function,
    sql_types::{Array, Nullable, SingleValue, Text},
};

define_sql_function! {
    #[sql_name = "COALESCE"]
    fn coalesce<T: SingleValue>(x: Nullable<T>, y: T) -> T;
}

define_sql_function! {
    #[sql_name = "ARRAY_AGG"]
    #[aggregate]
    fn array_agg<T: SingleValue>(x: T) -> Array<T>
}

define_sql_function! {
    #[sql_name = "ARRAY_REMOVE"]
    fn array_remove<T: SingleValue>(arr: Nullable<Array<T>>, value: T) -> Nullable<Array<T>>
}

define_sql_function! {
    #[sql_name = "array_deduplicate"]
    /// Custom function only made available in search service database by CREATE FUNCTION array_deduplicate...
    fn array_deduplicate<T: SingleValue>(arr: Nullable<Array<T>>) -> Nullable<Array<T>>
}

define_sql_function! {
    #[sql_name = "unaccent"]
    /// Custom function only made available in search service database by CREATE EXTENSION unaccent;
    fn unaccent(dict: Text, text: Text) -> Text
}
