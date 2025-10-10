#![allow(clippy::type_complexity)]

use diesel::{
    define_sql_function,
    expression::AsExpression,
    sql_types::{Array, Float8, Int4, Nullable, SingleValue, Text},
};
use postgis_diesel::sql_types::Geography;

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

// Declare the function signature for
define_sql_function!(
    #[sql_name = "ST_Distance"]
    /// PostGIS's ST_Distance function that returns distance in meters (f64 in Rust)
    fn st_distance_inner(
        geom_a: Geography,
        geom_b: Geography,
    ) -> Float8
);

define_sql_function!(
    #[sql_name = "ST_DWithin"]
    /// PostGIS's ST_DWithin that determines if two geometries or geographies are within a specified distance in meters
    fn st_dwithin_inner(
        geom_a: Geography,
        geom_b: Geography,
        radius: Float8,
    ) -> Bool
);

define_sql_function!(
    #[sql_name = "ST_MakePoint"]
    /// PostGIS's ST_MakePoint that constructs a Geomgraphy object of type **POINT**
    fn st_makepoint(
        lon: Float8,
        lat: Float8
    ) -> Geography
);

define_sql_function!(
    #[sql_name = "ST_SetSRID"]
    /// PostGIS's ST_SetSRID that sets the Spatial Reference Identifier (SRID) for a given Geography object
    fn st_setsrid(
        geom: Geography,
        srid: Int4,
    ) -> Geography
);

/// The SRID constant for WGS 84
const SRID: i32 = 4326;

/// Calculates the **geodesic distance in meters** between two Geography objects.
///
/// This function is a wrapper for a PostGIS expression chain that automatically:
/// 1. Tags both objects with **SRID 4326** (WGS 84).
/// 2. Executes the underlying `ST_Distance(GEOGRAPHY, GEOGRAPHY)` function.
///
/// A Diesel expression representing the distance in **meters** (mapped to Rust `f64`).
pub fn st_distance_4326<
    T: diesel::expression::AsExpression<Geography>,
    U: diesel::expression::AsExpression<Geography>,
>(
    geom_a: T,
    geom_b: U,
) -> st_distance_inner<
    st_setsrid<T, <i32 as AsExpression<Int4>>::Expression>,
    st_setsrid<U, <i32 as AsExpression<Int4>>::Expression>,
> {
    st_distance_inner(st_setsrid(geom_a, SRID), st_setsrid(geom_b, SRID))
}

/// Determines if two Geography objects are within a specified distance in meters
///
/// This function is a wrapper for a PostGIS expression chain that automatically:
/// 1. Tags both objects with **SRID 4326** (WGS 84).
/// 2. Executes the underlying `ST_DWithin(GEOGRAPHY, GEOGRAPHY, radius)` function.
///
/// A Diesel expression representing the result of the check (mapped to Rust `bool`).
pub fn st_dwithin_4326<
    T: diesel::expression::AsExpression<Geography>,
    U: diesel::expression::AsExpression<Geography>,
    R: diesel::expression::AsExpression<Float8>,
>(
    geom_a: T,
    geom_b: U,
    radius: R,
) -> st_dwithin_inner<
    st_setsrid<T, <i32 as AsExpression<Int4>>::Expression>,
    st_setsrid<U, <i32 as AsExpression<Int4>>::Expression>,
    R,
> {
    st_dwithin_inner(st_setsrid(geom_a, SRID), st_setsrid(geom_b, SRID), radius)
}
