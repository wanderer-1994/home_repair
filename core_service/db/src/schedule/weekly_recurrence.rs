use crate::schema::schedule_weekly_recurrence;
use chrono::{NaiveDateTime, NaiveTime};
use db_utils::AsyncPgConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use entity_type::{ScheduleId, ScheduleType, Weekday};
use error::Result;

#[derive(Debug)]
pub struct ScheduleWeeklyRecurrence {
    pub schedule_id: ScheduleId,
    pub weekday_times: Vec<WeekdayTime>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schedule_weekly_recurrence)]
pub struct WeekdayTime {
    pub id: i64,
    pub schedule_id: ScheduleId,
    pub schedule_type: ScheduleType,
    pub weekday: Weekday,
    pub times: Vec<NaiveTime>,
    pub updated_at: NaiveDateTime,
}

impl ScheduleWeeklyRecurrence {
    pub(crate) async fn create(
        schedule_id: ScheduleId,
        new: NewWeeklyRecurrenceSchedule,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        let insertable_records = new
            .weekday_times
            .iter()
            .map(|r| WeeklyRecurrenceScheduleInsertable {
                schedule_id,
                weekday: r.weekday,
                times: &r.times,
            })
            .collect::<Vec<_>>();

        let weekday_times = diesel::insert_into(schedule_weekly_recurrence::table)
            .values(insertable_records)
            .get_results::<WeekdayTime>(conn)
            .await?;

        Ok(ScheduleWeeklyRecurrence {
            schedule_id,
            weekday_times,
        })
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schedule_weekly_recurrence)]
struct WeeklyRecurrenceScheduleInsertable<'a> {
    schedule_id: ScheduleId,
    weekday: Weekday,
    times: &'a [NaiveTime],
}

#[derive(Debug)]
pub struct NewWeeklyRecurrenceSchedule {
    pub weekday_times: Vec<NewWeekdayTime>,
}

#[derive(Debug)]
pub struct NewWeekdayTime {
    pub weekday: Weekday,
    pub times: Vec<NaiveTime>,
}
