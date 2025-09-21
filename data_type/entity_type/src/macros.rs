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
