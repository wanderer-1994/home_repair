use super::AccountService;
use account_service_db as db;
use actor_auth::ActorAuth;
use db_utils::with_readonly_db;
use entity_type::HandymanId;
use error::Result;
use scoped_futures::ScopedFutureExt;

impl AccountService {
    #[tracing::instrument(skip(self))]
    pub async fn load_handyman_account_by_ids(
        &self,
        request: LoadHandymanAccountByIdsRequest,
    ) -> Result<LoadHandymanAccountByIdsResponse> {
        let LoadHandymanAccountByIdsRequest {
            actor_auth,
            account_ids,
        } = request;
        let handymans = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::HandymanAccount::load_by_ids(&actor_auth, &account_ids, conn).scope_boxed()
        })
        .await?;

        Ok(LoadHandymanAccountByIdsResponse { handymans })
    }

    #[tracing::instrument(skip(self))]
    pub async fn load_handyman_profile_by_ids(
        &self,
        request: LoadHandymanProfileByIdsRequest,
    ) -> Result<LoadHandymanProfileByIdsResponse> {
        let LoadHandymanProfileByIdsRequest {
            actor_auth,
            account_ids,
        } = request;
        let profiles = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::HandymanProfile::load_by_ids(&actor_auth, &account_ids, conn).scope_boxed()
        })
        .await?;

        Ok(LoadHandymanProfileByIdsResponse { profiles })
    }
}

#[derive(Debug)]
pub struct LoadHandymanAccountByIdsRequest {
    pub actor_auth: ActorAuth,
    pub account_ids: Vec<HandymanId>,
}

#[derive(Debug)]
pub struct LoadHandymanAccountByIdsResponse {
    pub handymans: Vec<db::HandymanAccount>,
}

#[derive(Debug)]
pub struct LoadHandymanProfileByIdsRequest {
    pub actor_auth: ActorAuth,
    pub account_ids: Vec<HandymanId>,
}

#[derive(Debug)]
pub struct LoadHandymanProfileByIdsResponse {
    pub profiles: Vec<db::HandymanProfile>,
}
