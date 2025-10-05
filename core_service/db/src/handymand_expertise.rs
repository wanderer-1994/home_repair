use std::ops::Deref;

use actor_auth::ActorAuth;
use chrono::NaiveDateTime;
use db_utils::AsyncPgConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use entity_type::{
    HandymanAccessGuardId, HandymanExpertiseId, HandymanId, ServiceLayer1, ServiceLayer2,
};
use error::Result;
use share_service_schema::handyman_expertise;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = handyman_expertise)]
pub struct HandymanExpertise {
    pub id: HandymanExpertiseId,
    pub handyman_id: HandymanId,
    pub service: ServiceLayer2,
    pub note: Option<String>,
    pub rate_vnd: Option<i32>,
    pub created_at: NaiveDateTime,
}

impl HandymanExpertise {
    pub async fn create_many(
        actor_auth: &ActorAuth,
        handyman_id: HandymanId,
        new_records: &[NewHandymanExpertise<'_>],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>> {
        actor_auth.require_handyman_access(handyman_id)?;

        let results = diesel::insert_into(handyman_expertise::table)
            .values(
                new_records
                    .iter()
                    .map(|record| (record, handyman_expertise::handyman_id.eq(handyman_id)))
                    .collect::<Vec<_>>(),
            )
            .on_conflict_do_nothing()
            .returning(Self::as_returning())
            .get_results::<Self>(conn)
            .await?;

        Ok(results)
    }

    /// Returns list of expertise belonging to a handyman.
    /// This API requires god or admin or any session actor.
    pub async fn get_by_handyman(
        _actor_auth: &ActorAuth,
        handyman_id: HandymanId,
        conn: &mut AsyncPgConnection,
    ) -> Result<HandymanExpertiseList> {
        let result = handyman_expertise::table
            .filter(handyman_expertise::handyman_id.eq(handyman_id))
            .select(Self::as_select())
            .order(handyman_expertise::service)
            .load::<Self>(conn)
            .await?;

        Ok(HandymanExpertiseList(result))
    }

    pub async fn update(
        actor_auth: &ActorAuth,
        HandymanAccessGuardId {
            handyman_id,
            entity_id,
        }: HandymanAccessGuardId<HandymanExpertiseId>,
        changeset: HandymanExpertiseChangeset<'_>,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        actor_auth.require_handyman_access(handyman_id)?;

        let result = diesel::update(
            handyman_expertise::table.filter(
                handyman_expertise::id
                    .eq(entity_id)
                    .and(handyman_expertise::handyman_id.eq(handyman_id)),
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
        ids_to_delete: &[HandymanExpertiseId],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<HandymanExpertiseId>> {
        actor_auth.require_handyman_access(handyman_id)?;

        let deleted_ids = diesel::delete(
            handyman_expertise::table.filter(
                handyman_expertise::handyman_id
                    .eq(handyman_id)
                    .and(handyman_expertise::id.eq_any(ids_to_delete)),
            ),
        )
        .returning(handyman_expertise::id)
        .get_results::<HandymanExpertiseId>(conn)
        .await?;

        db_utils::validate_rows_affected(
            "handyman_expertise",
            ids_to_delete.len(),
            deleted_ids.len(),
        )?;

        Ok(deleted_ids)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = handyman_expertise)]
pub struct NewHandymanExpertise<'a> {
    pub service: ServiceLayer2,
    pub note: Option<&'a str>,
    pub rate_vnd: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = handyman_expertise)]
pub struct HandymanExpertiseChangeset<'a> {
    pub note: Option<Option<&'a str>>,
    pub rate_vnd: Option<Option<i32>>,
}

pub struct HandymanExpertiseList(Vec<HandymanExpertise>);

impl Deref for HandymanExpertiseList {
    type Target = Vec<HandymanExpertise>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HandymanExpertiseList {
    pub fn into_group(self) -> Vec<HandymanExpertiseGroup> {
        db_utils::group_by(self.0.into_iter().map(|e| (e.service.layer1(), e)))
            .into_iter()
            .map(|(group, expertises)| HandymanExpertiseGroup { group, expertises })
            .collect()
    }
}

pub struct HandymanExpertiseGroup {
    pub group: ServiceLayer1,
    pub expertises: Vec<HandymanExpertise>,
}
