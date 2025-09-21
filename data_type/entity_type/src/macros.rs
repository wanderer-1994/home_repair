#[macro_export]
macro_rules! def_id_newtype {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
        #[cfg_attr(feature = "db", derive(diesel_derive_newtype::DieselNewType))]
        pub struct $name(pub i64);
    };

    ($($name:ident,)+) => {
        $(
            def_id_newtype!{$name}
        )+
    }
}

/// The same as `define_enum!` but also derives `juniper::GraphQLEnum`.
#[macro_export]
macro_rules! define_graphql_enum {
    (
        PgType = $pg_type:literal,
        $name:ident $(#[doc = $enum_doc:expr])?,
        $($variant:ident $(#[doc = $doc:expr])?,)+
    ) => {
        $(#[doc = $enum_doc])?
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(
            feature = "db",
            derive(diesel_derive_enum::DbEnum),
            PgType = $pg_type,
            DbValueStyle = "SCREAMING_SNAKE_CASE"
        )]
        #[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
        pub enum $name {
            $(
                $(#[doc = $doc])?
                $variant,
            )+
        }
    };
}
