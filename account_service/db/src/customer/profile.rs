use crate::schema::customer_profile;
use actor_auth::ActorAuth;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use entity_type::CustomerId;
use error::{Error, Result};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = customer_profile)]
pub struct CustomerProfile {
    pub customer_id: CustomerId,
    pub nick_name: String,
}

impl CustomerProfile {
    pub async fn create(
        actor_auth: &ActorAuth,
        customer_id: CustomerId,
        new_profile: NewCustomerProfile<'_>,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        actor_auth.require_customer_access(customer_id)?;
        new_profile.validate()?;

        diesel::insert_into(customer_profile::table)
            .values((new_profile, customer_profile::customer_id.eq(customer_id)))
            .returning(Self::as_select())
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }

    /// Load many profiles by ids
    pub async fn load_by_ids(
        // TODO: define read permission for customer profile
        _actor_auth: &ActorAuth,
        ids: &[CustomerId],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>> {
        let result = customer_profile::table
            .filter(customer_profile::customer_id.eq_any(ids))
            .select(Self::as_select())
            .load::<Self>(conn)
            .await?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, Insertable)]
#[diesel(table_name = customer_profile)]
pub struct NewCustomerProfile<'a> {
    pub nick_name: &'a str,
}

impl<'a> NewCustomerProfile<'a> {
    fn validate(&self) -> Result<()> {
        typesafe::require_trimmed_and_not_empty_str(self.nick_name, "nick_name")
    }
}
