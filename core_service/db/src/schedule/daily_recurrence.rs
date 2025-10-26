use crate::schema::schedule_daily_recurrence;
use chrono::{NaiveDateTime, NaiveTime};
use db_utils::AsyncPgConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use entity_type::{ScheduleId, ScheduleType};
use error::{Error, Result};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schedule_daily_recurrence)]
pub struct ScheduleDailyRecurrence {
    pub id: ScheduleId,
    pub schedule_type: ScheduleType,
    pub times: Vec<NaiveTime>,
    pub updated_at: NaiveDateTime,
}

impl ScheduleDailyRecurrence {
    pub(crate) async fn create(
        schedule_id: ScheduleId,
        new: NewDailyRecurrenceSchedule,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        diesel::insert_into(schedule_daily_recurrence::table)
            .values((schedule_daily_recurrence::id.eq(schedule_id), new))
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schedule_daily_recurrence)]
pub struct NewDailyRecurrenceSchedule {
    pub times: Vec<NaiveTime>,
}
