use crate::{
    NewDailyRecurrenceSchedule, NewFixedTimeSchedule, NewWeeklyRecurrenceSchedule,
    ScheduleDailyRecurrence, ScheduleFixedTime, ScheduleWeeklyRecurrence, schema::schedule,
};
use actor_auth::ActorAuth;
use chrono::NaiveDateTime;
use db_utils::AsyncPgConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use entity_type::{ScheduleId, ScheduleType};
use error::{Error, Result};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schedule)]
pub struct ScheduleBase {
    pub id: ScheduleId,
    pub schedule_type: ScheduleType,
    pub created_at: NaiveDateTime,
}

impl ScheduleBase {
    pub(crate) async fn create(
        schedule_type: ScheduleType,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        diesel::insert_into(schedule::table)
            .values(schedule::schedule_type.eq(schedule_type))
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub enum ScheduleVariant {
    FixedTime(ScheduleFixedTime),
    DailyRecurrence(ScheduleDailyRecurrence),
    WeeklyRecurrence(ScheduleWeeklyRecurrence),
}

#[derive(Debug)]
pub struct Schedule {
    pub base: ScheduleBase,
    pub variant: ScheduleVariant,
}

impl Schedule {
    /// Create a new schedule. Requires authenticated session.
    pub async fn create(
        _actor_auth: &ActorAuth,
        new: NewScheduleVariant,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        let base = ScheduleBase::create(new.schedule_type(), conn).await?;

        let result = match new {
            NewScheduleVariant::FixedTime(fixed_time) => {
                let variant = ScheduleFixedTime::create(base.id, fixed_time, conn).await?;
                Schedule {
                    base,
                    variant: ScheduleVariant::FixedTime(variant),
                }
            }
            NewScheduleVariant::DailyRecurrence(daily_recurrence) => {
                let variant =
                    ScheduleDailyRecurrence::create(base.id, daily_recurrence, conn).await?;
                Schedule {
                    base,
                    variant: ScheduleVariant::DailyRecurrence(variant),
                }
            }
            NewScheduleVariant::WeeklyRecurrence(weekly_recurrence) => {
                let variant =
                    ScheduleWeeklyRecurrence::create(base.id, weekly_recurrence, conn).await?;
                Schedule {
                    base,
                    variant: ScheduleVariant::WeeklyRecurrence(variant),
                }
            }
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub enum NewScheduleVariant {
    FixedTime(NewFixedTimeSchedule),
    DailyRecurrence(NewDailyRecurrenceSchedule),
    WeeklyRecurrence(NewWeeklyRecurrenceSchedule),
}

impl NewScheduleVariant {
    pub fn schedule_type(&self) -> ScheduleType {
        match self {
            NewScheduleVariant::FixedTime(_) => ScheduleType::FixedTime,
            NewScheduleVariant::DailyRecurrence(_) => ScheduleType::DailyRecurrence,
            NewScheduleVariant::WeeklyRecurrence(_) => ScheduleType::WeeklyRecurrence,
        }
    }
}
