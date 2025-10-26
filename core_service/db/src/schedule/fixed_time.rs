use crate::schema::schedule_fixed_time;
use chrono::NaiveDateTime;
use db_utils::AsyncPgConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use entity_type::{ScheduleId, ScheduleType};
use error::{Error, Result};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schedule_fixed_time)]
pub struct ScheduleFixedTime {
    pub id: ScheduleId,
    pub schedule_type: ScheduleType,
    pub time: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ScheduleFixedTime {
    pub(crate) async fn create(
        schedule_id: ScheduleId,
        new: NewFixedTimeSchedule,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        diesel::insert_into(schedule_fixed_time::table)
            .values((schedule_fixed_time::id.eq(schedule_id), new))
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schedule_fixed_time)]
pub struct NewFixedTimeSchedule {
    pub time: NaiveDateTime,
}
