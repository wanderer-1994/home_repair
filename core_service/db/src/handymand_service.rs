use std::ops::Deref;

use crate::schema::handyman_service;
use actor_auth::ActorAuth;
use chrono::NaiveDateTime;
use db_utils::AsyncPgConnection;
use diesel::{dsl::exists, prelude::*};
use diesel_async::RunQueryDsl;
use entity_type::{
    HandymanAccessGuardId, HandymanId, HandymanServiceId, ServiceLayer1, ServiceLayer2,
};
use error::{Error, Result};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = handyman_service)]
pub struct HandymanService {
    pub id: HandymanServiceId,
    pub handyman_id: HandymanId,
    pub service: ServiceLayer2,
    pub note: Option<String>,
    pub rate_vnd: Option<i32>,
    pub created_at: NaiveDateTime,
}

impl HandymanService {
    pub async fn create_many(
        actor_auth: &ActorAuth,
        handyman_id: HandymanId,
        new_records: &[NewHandymanService<'_>],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>> {
        actor_auth.require_handyman_access(handyman_id)?;

        let results = diesel::insert_into(handyman_service::table)
            .values(
                new_records
                    .iter()
                    .map(|record| (record, handyman_service::handyman_id.eq(handyman_id)))
                    .collect::<Vec<_>>(),
            )
            .on_conflict_do_nothing()
            .returning(Self::as_returning())
            .get_results::<Self>(conn)
            .await?;

        Ok(results)
    }

    /// Returns list of service belonging to a handyman.
    /// This API requires god or admin or any session actor.
    pub async fn get_by_handyman(
        _actor_auth: &ActorAuth,
        handyman_id: HandymanId,
        conn: &mut AsyncPgConnection,
    ) -> Result<HandymanServiceList> {
        let result = handyman_service::table
            .filter(handyman_service::handyman_id.eq(handyman_id))
            .select(Self::as_select())
            .order(handyman_service::service)
            .load::<Self>(conn)
            .await?;

        Ok(HandymanServiceList(result))
    }

    pub async fn update(
        actor_auth: &ActorAuth,
        HandymanAccessGuardId {
            handyman_id,
            entity_id,
        }: HandymanAccessGuardId<HandymanServiceId>,
        changeset: HandymanServiceChangeset<'_>,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        actor_auth.require_handyman_access(handyman_id)?;

        let result = diesel::update(
            handyman_service::table.filter(
                handyman_service::id
                    .eq(entity_id)
                    .and(handyman_service::handyman_id.eq(handyman_id)),
            ),
        )
        .set(changeset)
        .get_result::<Self>(conn)
        .await?;

        Ok(result)
    }

    pub async fn delete_many(
        actor_auth: &ActorAuth,
        handyman_id: HandymanId,
        ids_to_delete: &[HandymanServiceId],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>> {
        actor_auth.require_handyman_access(handyman_id)?;

        let records = diesel::delete(
            handyman_service::table.filter(
                handyman_service::handyman_id
                    .eq(handyman_id)
                    .and(handyman_service::id.eq_any(ids_to_delete)),
            ),
        )
        .returning(Self::as_returning())
        .get_results::<Self>(conn)
        .await?;

        db_utils::validate_rows_affected("handyman_service", ids_to_delete.len(), records.len())?;

        Ok(records)
    }

    pub async fn handyman_service_exists(
        handyman_id: HandymanId,
        service: ServiceLayer2,
        conn: &mut AsyncPgConnection,
    ) -> Result<bool> {
        diesel::select(exists(
            handyman_service::table.filter(
                handyman_service::handyman_id
                    .eq(handyman_id)
                    .and(handyman_service::service.eq(service)),
            ),
        ))
        .get_result::<bool>(conn)
        .await
        .map_err(Error::from)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = handyman_service)]
pub struct NewHandymanService<'a> {
    pub service: ServiceLayer2,
    pub note: Option<&'a str>,
    pub rate_vnd: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = handyman_service)]
pub struct HandymanServiceChangeset<'a> {
    pub note: Option<Option<&'a str>>,
    pub rate_vnd: Option<Option<i32>>,
}

pub struct HandymanServiceList(Vec<HandymanService>);

impl Deref for HandymanServiceList {
    type Target = Vec<HandymanService>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HandymanServiceList {
    pub fn into_group(self) -> Vec<HandymanServiceGroup> {
        db_utils::group_by(self.0.into_iter().map(|e| (e.service.layer1(), e)))
            .into_iter()
            .map(|(group, services)| HandymanServiceGroup { group, services })
            .collect()
    }
}

pub struct HandymanServiceGroup {
    pub group: ServiceLayer1,
    pub services: Vec<HandymanService>,
}
