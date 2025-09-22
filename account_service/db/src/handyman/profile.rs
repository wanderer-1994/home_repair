use actor_auth::ActorAuth;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use entity_type::HandymanId;
use error::{Error, Result};
use share_service_schema::handyman_profile;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = handyman_profile)]
pub struct HandymanProfile {
    pub handyman_id: HandymanId,
    pub first_name: String,
    pub last_name: String,
}

impl HandymanProfile {
    pub async fn create(
        actor_auth: &ActorAuth,
        handyman_id: HandymanId,
        new_profile: NewHandymanProfile<'_>,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        actor_auth.require_handyman_access(handyman_id)?;
        new_profile.validate()?;

        diesel::insert_into(handyman_profile::table)
            .values(new_profile)
            .returning(Self::as_select())
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }
}

#[derive(Debug, Clone, Copy, Insertable)]
#[diesel(table_name = handyman_profile)]
pub struct NewHandymanProfile<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}

impl<'a> NewHandymanProfile<'a> {
    fn validate(&self) -> Result<()> {
        crate::require_trimmed_and_not_empty_str(self.first_name, "first_name")?;
        crate::require_trimmed_and_not_empty_str(self.last_name, "last_name")
    }
}
