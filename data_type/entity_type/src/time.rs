use crate::define_graphql_enum;

define_graphql_enum!(
    PgType = "text",
    Weekday #[doc = "The day of week."],
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
);

impl From<Weekday> for chrono::Weekday {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Mon => chrono::Weekday::Mon,
            Weekday::Tue => chrono::Weekday::Tue,
            Weekday::Wed => chrono::Weekday::Wed,
            Weekday::Thu => chrono::Weekday::Thu,
            Weekday::Fri => chrono::Weekday::Fri,
            Weekday::Sat => chrono::Weekday::Sat,
            Weekday::Sun => chrono::Weekday::Sun,
        }
    }
}

impl From<chrono::Weekday> for Weekday {
    fn from(value: chrono::Weekday) -> Self {
        match value {
            chrono::Weekday::Mon => Weekday::Mon,
            chrono::Weekday::Tue => Weekday::Tue,
            chrono::Weekday::Wed => Weekday::Wed,
            chrono::Weekday::Thu => Weekday::Thu,
            chrono::Weekday::Fri => Weekday::Fri,
            chrono::Weekday::Sat => Weekday::Sat,
            chrono::Weekday::Sun => Weekday::Sun,
        }
    }
}
