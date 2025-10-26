use crate::{NewScheduleVariant, Schedule, schema::customer_task_request};
use actor_auth::ActorAuth;
use chrono::NaiveDateTime;
use db_utils::AsyncPgConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use entity_type::{CustomerId, CustomerTaskRequestId, ScheduleId, ServiceLayer2};
use error::Result;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = customer_task_request)]
pub struct CustomerTaskRequest {
    pub id: CustomerTaskRequestId,
    pub customer_id: CustomerId,
    pub service: ServiceLayer2,
    pub title: String,
    pub note: Option<String>,
    pub schedule: ScheduleId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CustomerTaskRequest {
    pub async fn create(
        actor_auth: &ActorAuth,
        NewCustomerTaskRequest {
            customer_id,
            service,
            title,
            note,
            schedule,
        }: NewCustomerTaskRequest,
        conn: &mut AsyncPgConnection,
    ) -> Result<(Self, Schedule)> {
        actor_auth.require_customer_access(customer_id)?;
        let schedule = Schedule::create(actor_auth, schedule, conn).await?;

        let new_request = CustomerTaskRequestInsertable {
            customer_id,
            service,
            title,
            note,
            schedule: schedule.base.id,
        };

        let result = diesel::insert_into(customer_task_request::table)
            .values(new_request)
            .get_result::<Self>(conn)
            .await?;

        Ok((result, schedule))
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = customer_task_request)]
struct CustomerTaskRequestInsertable {
    customer_id: CustomerId,
    service: ServiceLayer2,
    title: String,
    note: Option<String>,
    schedule: ScheduleId,
}

pub struct NewCustomerTaskRequest {
    pub customer_id: CustomerId,
    pub service: ServiceLayer2,
    pub title: String,
    pub note: Option<String>,
    pub schedule: NewScheduleVariant,
}
