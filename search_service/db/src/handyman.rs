use crate::schema::handyman;
use db_utils::{AsyncPgConnection, PaginateOffset};
use diesel::{prelude::*, upsert::excluded};
use diesel_async::RunQueryDsl;
use diesel_full_text_search::{self as dfts, TsVectorExtensions};
use entity_type::{HandymanId, ServiceLayer2};
use error::Result;
use paging::{PagingOffsetConfig, PagingOffsetInfo, PagingOffsetPayload};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = handyman)]
pub struct HandymanSearch {
    pub handyman_id: HandymanId,
    pub full_name: Option<String>,
    pub skills: Option<Vec<Option<ServiceLayer2>>>,
}

impl HandymanSearch {
    pub async fn index_full_name(
        handyman_id: HandymanId,
        full_name: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        let result = diesel::insert_into(handyman::table)
            .values((
                handyman::handyman_id.eq(handyman_id),
                handyman::full_name.eq(full_name),
            ))
            .on_conflict(handyman::handyman_id)
            .do_update()
            .set(handyman::full_name.eq(excluded(handyman::full_name)))
            .returning(Self::as_returning())
            .get_result(conn)
            .await?;

        Ok(result)
    }

    pub async fn index_add_skills(
        handyman_id: HandymanId,
        skills: &[ServiceLayer2],
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        let result = diesel::insert_into(handyman::table)
            .values((
                handyman::handyman_id.eq(handyman_id),
                handyman::skills.eq(skills),
            ))
            .on_conflict(handyman::handyman_id)
            .do_update()
            .set(handyman::skills.eq(db_utils::array_deduplicate(
                handyman::skills.concat(excluded(handyman::skills)),
            )))
            .returning(Self::as_returning())
            .get_result(conn)
            .await?;

        Ok(result)
    }

    pub async fn index_remove_skill(
        handyman_id: HandymanId,
        skill: ServiceLayer2,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>> {
        let result = diesel::update(handyman::table.filter(handyman::handyman_id.eq(handyman_id)))
            .set(handyman::skills.eq(db_utils::array_remove(handyman::skills, skill)))
            .returning(Self::as_returning())
            .get_result(conn)
            .await
            .optional()?;

        Ok(result)
    }

    pub async fn delete_index(
        handyman_id: HandymanId,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>> {
        let result = diesel::delete(handyman::table.filter(handyman::handyman_id.eq(handyman_id)))
            .returning(Self::as_returning())
            .get_result(conn)
            .await
            .optional()?;

        Ok(result)
    }

    /// Search handyman order by ranking desc
    pub async fn search(
        HandymanSearchFilter {
            handyman_ids,
            name,
            skills,
        }: HandymanSearchFilter,
        paging_config: PagingOffsetConfig,
        conn: &mut AsyncPgConnection,
    ) -> Result<PagingOffsetPayload<HandymanId>> {
        let mut query = handyman::table.select(handyman::handyman_id).into_boxed();

        if let Some(handyman_ids) = handyman_ids {
            query = query.filter(handyman::handyman_id.eq_any(handyman_ids));
        }

        if let Some(name) = name {
            let unaccented_name = db_utils::unaccent("simple", name);
            query = query.filter(
                handyman::search_vector.is_not_null().and(
                    handyman::search_vector
                        .assume_not_null()
                        .matches(dfts::plainto_tsquery(unaccented_name)),
                ),
            );
        }

        if let Some(skills) = skills {
            query = query.filter(handyman::skills.overlaps_with(skills));
        }

        let (result, total_count) = query
            .paginate_offset(paging_config)
            .load_and_count_total::<HandymanId>(conn)
            .await?;

        Ok(PagingOffsetPayload {
            paging_info: PagingOffsetInfo {
                page: paging_config.page,
                page_size: paging_config.page_size,
                total_count,
            },
            items: result,
        })
    }
}

#[derive(Debug)]
/// Filter for handyman search. Fields are AND condition.
pub struct HandymanSearchFilter {
    /// OR condition on handyman IDs
    pub handyman_ids: Option<Vec<HandymanId>>,
    pub name: Option<String>,
    /// OR condition on handyman skills
    pub skills: Option<Vec<ServiceLayer2>>,
}
