use async_graphql::InputObject;
use chrono::{NaiveDateTime, NaiveTime};
use entity_type::Weekday;

#[derive(Debug, InputObject)]
/// Include location and time for a schedule
pub struct ScheduleInput {
    pub location: LocationInput,
    pub time: ScheduleTimeInput,
}

#[derive(Debug, InputObject)]
pub struct LocationInput {
    pub city: String,
    pub address_line1: String,
    pub formatted_address: String,
    pub corrdinates: GeoCoordinates,
}

#[derive(Debug, InputObject)]
pub struct GeoCoordinates {
    pub lon: i64,
    pub lat: i64,
}

#[derive(Debug, InputObject)]
/// Defines the rule for when an event or task is scheduled.
/// Only one field should be not-null.
pub struct ScheduleTimeInput {
    /// A singular, non-repeating date and time.
    pub fixed_time: Option<FixedTime>,
    /// A time of day that repeats on specified days (e.g., Mon, Wed, Fri at 9:00 AM).
    pub daily_recurrence: Option<DailyRecurrence>,
    /// A rule that repeats based on the day of the week, often with a start/end date.
    pub weekly_recurrence: Option<WeeklyRecurrence>,
}

#[derive(Debug, InputObject)]
pub struct FixedTime {
    pub time: NaiveDateTime,
}

#[derive(Debug, InputObject)]
pub struct DailyRecurrence {
    pub times: Vec<NaiveTime>,
}

#[derive(Debug, InputObject)]
pub struct WeeklyRecurrence {
    pub times: Vec<WeekdayTime>,
}

#[derive(Debug, InputObject)]
pub struct WeekdayTime {
    pub day: Weekday,
    pub times: Vec<NaiveTime>,
}
