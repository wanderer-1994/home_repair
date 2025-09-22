use actor_auth::ActorAuth;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use entity_type::CustomerId;
use error::{Error, Result};
use share_service_schema::customer_profile;

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
            .values(new_profile)
            .returning(Self::as_select())
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }
}

#[derive(Debug, Clone, Copy, Insertable)]
#[diesel(table_name = customer_profile)]
pub struct NewCustomerProfile<'a> {
    pub nick_name: &'a str,
}

impl<'a> NewCustomerProfile<'a> {
    fn validate(&self) -> Result<()> {
        crate::require_trimmed_and_not_empty_str(self.nick_name, "nick_name")
    }
}
