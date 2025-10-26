use crate::define_graphql_enum;

define_graphql_enum!(
    PgType = "text",
    ScheduleType #[doc = "Type of schedule"],
    FixedTime,
    DailyRecurrence,
    WeeklyRecurrence,
);
