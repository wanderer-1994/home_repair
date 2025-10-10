use async_graphql::{Context, ID, InputObject, Object};
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::{GlobalId, Handyman, PagingOffsetPayload};
use entity_type::ServiceLayer2;
use error::{Error, Result};
use paging::{PagingOffsetConfig, PagingOffsetInput};
use search_service_db as sea_db;
use search_service_server::HandymanSearchRequest;

#[derive(Default)]
pub struct HandymanDiscoveryQuery;

#[Object]
impl HandymanDiscoveryQuery {
    #[tracing::instrument(skip(self, ctx))]
    async fn handyman_search(
        &self,
        ctx: &Context<'_>,
        filter: HandymanSearchFilter,
        paging_config: PagingOffsetInput,
    ) -> Result<PagingOffsetPayload<Handyman>> {
        let context = ctx.data::<RequestContext>()?;
        context.try_session_context().await?;

        let data = context
            .search_service_client
            .handyman_search(HandymanSearchRequest {
                filter: sea_db::HandymanSearchFilter::try_from(filter)?,
                paging_config: PagingOffsetConfig::try_from(paging_config)?,
            })
            .await?
            .result;

        Ok(PagingOffsetPayload {
            paging_info: data.paging_info,
            items: data.items.into_iter().map(Handyman::new).collect(),
        })
    }
}

#[derive(Debug, InputObject)]
pub struct HandymanSearchFilter {
    pub services: Option<Vec<ServiceLayer2>>,
    pub name: Option<String>,
    pub ids: Option<Vec<ID>>,
}

impl TryFrom<HandymanSearchFilter> for sea_db::HandymanSearchFilter {
    type Error = Error;

    fn try_from(
        HandymanSearchFilter {
            services,
            name,
            ids,
        }: HandymanSearchFilter,
    ) -> Result<Self> {
        let handyman_ids = if let Some(ids) = ids {
            Some(
                ids.iter()
                    .map(|id| Handyman::from_global_id(id).map(|n| n.inner_id()))
                    .collect::<Result<Vec<_>>>()?,
            )
        } else {
            None
        };

        let result = Self {
            handyman_ids,
            name,
            skills: services,
            distance_within: None,
        };
        Ok(result)
    }
}
