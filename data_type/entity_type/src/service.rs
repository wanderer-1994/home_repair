/// The def_services! macro generates the ServiceLayer1 and ServiceLayer2 enums,
/// along with the layer1 and layer2 conversion methods for convenience.
macro_rules! def_services {
    (
        $(
            $l1_parent:ident (
                $( $l2_children:ident, )*
            ),
        )+
    ) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize, strum_macros::EnumIter,
        )]
        #[cfg_attr(
            feature = "db",
            derive(diesel_derive_enum::DbEnum),
            PgType = "text",
            DbValueStyle = "SCREAMING_SNAKE_CASE"
        )]
        #[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
        pub enum ServiceLayer1 {
            $( $l1_parent ),*
        }

        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize, strum_macros::EnumIter,
        )]
        #[cfg_attr(
            feature = "db",
            derive(diesel_derive_enum::DbEnum),
            PgType = "text",
            DbValueStyle = "SCREAMING_SNAKE_CASE"
        )]
        #[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
        pub enum ServiceLayer2 {
            $(
                $( $l2_children ),*
            ),*
        }

        impl ServiceLayer2 {
            /// Returns the parent [`ServiceLayer1`] for the current [`ServiceLayer2`].
            pub fn layer1(&self) -> ServiceLayer1 {
                match self {
                    $(
                        $(
                            ServiceLayer2::$l2_children => ServiceLayer1::$l1_parent,
                        )*
                    )*
                }
            }
        }

        impl ServiceLayer1 {
            /// Returns a slice containing all child [`ServiceLayer2`] variants.
            pub fn layer2(&self) -> &[ServiceLayer2] {
                match self {
                    $(
                        ServiceLayer1::$l1_parent => &[
                            $(
                                ServiceLayer2::$l2_children
                            ),*
                        ],
                    )*
                }
            }
        }
    };
}

def_services!(
    AirConditioner(AirConditionerFixing, AirConditionerCleaning,),
    WashingMachine(WashingMachineFixing, WashingMachineCleaning,),
    Other(Other,),
);
