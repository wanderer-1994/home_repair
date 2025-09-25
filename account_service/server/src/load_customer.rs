use super::AccountService;
use account_service_db as db;
use actor_auth::ActorAuth;
use db_utils::with_readonly_db;
use entity_type::CustomerId;
use error::Result;
use scoped_futures::ScopedFutureExt;

impl AccountService {
    #[tracing::instrument(skip(self))]
    pub async fn load_customer_account_by_ids(
        &self,
        request: LoadCustomerAccountByIdsRequest,
    ) -> Result<LoadCustomerAccountByIdsResponse> {
        let LoadCustomerAccountByIdsRequest {
            actor_auth,
            account_ids,
        } = request;
        let customers = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::CustomerAccount::load_by_ids(&actor_auth, &account_ids, conn).scope_boxed()
        })
        .await?;

        Ok(LoadCustomerAccountByIdsResponse { customers })
    }

    #[tracing::instrument(skip(self))]
    pub async fn load_customer_profile_by_ids(
        &self,
        request: LoadCustomerProfileByIdsRequest,
    ) -> Result<LoadCustomerProfileByIdsResponse> {
        let LoadCustomerProfileByIdsRequest {
            actor_auth,
            account_ids,
        } = request;
        let profiles = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::CustomerProfile::load_by_ids(&actor_auth, &account_ids, conn).scope_boxed()
        })
        .await?;

        Ok(LoadCustomerProfileByIdsResponse { profiles })
    }
}

#[derive(Debug)]
pub struct LoadCustomerAccountByIdsRequest {
    pub actor_auth: ActorAuth,
    pub account_ids: Vec<CustomerId>,
}

#[derive(Debug)]
pub struct LoadCustomerAccountByIdsResponse {
    pub customers: Vec<db::CustomerAccount>,
}

#[derive(Debug)]
pub struct LoadCustomerProfileByIdsRequest {
    pub actor_auth: ActorAuth,
    pub account_ids: Vec<CustomerId>,
}

#[derive(Debug)]
pub struct LoadCustomerProfileByIdsResponse {
    pub profiles: Vec<db::CustomerProfile>,
}
