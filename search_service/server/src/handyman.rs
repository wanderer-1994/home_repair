use super::SearchService;
use db_utils::with_readonly_db;
use entity_type::{HandymanId, ServiceLayer2};
use error::Result;
use paging::{PagingConfig, PagingSearchPayload};
use scoped_futures::ScopedFutureExt;
use search_service_db as db;

impl SearchService {
    #[tracing::instrument(skip(self))]
    pub async fn handyman_index(
        &self,
        request: HandymanIndexRequest,
    ) -> Result<HandymanIndexResponse> {
        let HandymanIndexRequest {
            handyman_id,
            index_type,
        } = request;

        let index = with_readonly_db(&self.context.db_connection_pool, |conn| {
            async {
                let index = match index_type {
                    HandymanIndexType::SetFullName(full_name) => Some(
                        db::HandymanSearch::index_full_name(handyman_id, &full_name, conn).await?,
                    ),
                    HandymanIndexType::AddSkill(service) => {
                        Some(db::HandymanSearch::index_add_skill(handyman_id, service, conn).await?)
                    }
                    HandymanIndexType::RemoveSkill(service) => {
                        db::HandymanSearch::index_remove_skill(handyman_id, service, conn).await?
                    }
                };
                Ok(index)
            }
            .scope_boxed()
        })
        .await?;

        Ok(HandymanIndexResponse { index })
    }

    #[tracing::instrument(skip(self))]
    pub async fn handyman_index_delete(
        &self,
        request: HandymanIndexDeleteRequest,
    ) -> Result<HandymanIndexDeleteResponse> {
        let HandymanIndexDeleteRequest { handyman_id } = request;

        let index = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::HandymanSearch::delete_index(handyman_id, conn).scope_boxed()
        })
        .await?;

        Ok(HandymanIndexDeleteResponse { index })
    }

    #[tracing::instrument(skip(self))]
    pub async fn handyman_search(
        &self,
        request: HandymanSearchRequest,
    ) -> Result<HandymanSearchResponse> {
        let HandymanSearchRequest {
            filter,
            paging_config,
        } = request;
        let result = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::HandymanSearch::search(filter, paging_config, conn).scope_boxed()
        })
        .await?;

        Ok(HandymanSearchResponse { result })
    }
}

#[derive(Debug)]
pub struct HandymanIndexRequest {
    pub handyman_id: HandymanId,
    pub index_type: HandymanIndexType,
}

#[derive(Debug)]
pub enum HandymanIndexType {
    SetFullName(String),
    AddSkill(ServiceLayer2),
    RemoveSkill(ServiceLayer2),
}

#[derive(Debug)]
pub struct HandymanIndexResponse {
    pub index: Option<db::HandymanSearch>,
}

#[derive(Debug)]
pub struct HandymanIndexDeleteRequest {
    pub handyman_id: HandymanId,
}

#[derive(Debug)]
pub struct HandymanIndexDeleteResponse {
    pub index: Option<db::HandymanSearch>,
}

#[derive(Debug)]
pub struct HandymanSearchRequest {
    pub filter: db::HandymanSearchFilter,
    pub paging_config: PagingConfig,
}

#[derive(Debug)]
pub struct HandymanSearchResponse {
    pub result: PagingSearchPayload<HandymanId>,
}
