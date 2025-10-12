use crate::schema::handyman;
use db_utils::{AsyncPgConnection, PaginateOffset};
use diesel::{prelude::*, upsert::excluded};
use diesel_async::RunQueryDsl;
use diesel_full_text_search::{self as dfts, TsVectorExtensions};
use entity_type::{HandymanId, ServiceLayer2};
use error::{
    Error, Result,
    error_details::{BadRequest, bad_request::FieldViolation},
};
use paging::{PagingOffsetConfig, PagingOffsetInfo, PagingOffsetPayload};
use postgis_diesel::types::Point;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = handyman)]
pub struct HandymanSearch {
    pub handyman_id: HandymanId,
    pub full_name: Option<String>,
    pub skills: Option<Vec<Option<ServiceLayer2>>>,
    pub avg_rating_score: Option<i16>,
    pub location: Option<Point>,
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

    /// Search handyman order by ranking desc and location distance asc (if location filter is included).
    /// The resulting query looks like:
    /// SELECT "handyman"."handyman_id" FROM "handyman"
    ///     WHERE (
    ///         (
    ///             ("handyman"."handyman_id" = ANY('{1, 2, 3}'))
    ///                 AND
    ///             (("handyman"."search_vector" IS NOT NULL) AND "handyman"."search_vector" @@ plainto_tsquery(unaccent('simple', 'John')))
    ///                 AND
    ///             ("handyman"."skills" && '{AirConditionerFixing, WashingMachineFixing}')
    ///         )
    ///             AND
    ///         (("handyman"."location" IS NOT NULL) AND ST_DWithin(ST_SetSRID("handyman"."location", 4326),ST_SetSRID(ST_MakePoint(100.0, 90.0), 4326), 5000.0)
    ///     )
    ///     ORDER BY "handyman"."avg_rating_score" DESC, ST_Distance(ST_SetSRID("handyman"."location", 4326), ST_SetSRID(ST_MakePoint(100.0, 90.0), 4326)) ASC;
    pub async fn search(
        HandymanSearchFilter {
            handyman_ids,
            name,
            skills,
            distance_within,
        }: HandymanSearchFilter,
        paging_config: PagingOffsetConfig,
        conn: &mut AsyncPgConnection,
    ) -> Result<PagingOffsetPayload<HandymanId>> {
        let mut query = handyman::table
            .select(handyman::handyman_id)
            .order(handyman::avg_rating_score.desc())
            .into_boxed();

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

        if let Some(distance_within) = distance_within.map(|f| f.validate()).transpose()? {
            let point = db_utils::st_makepoint(distance_within.lon, distance_within.lat);
            query = query.filter(
                handyman::location
                    .is_not_null()
                    .and(db_utils::st_dwithin_4326(
                        handyman::location.assume_not_null(),
                        point,
                        distance_within.within_meters,
                    )),
            );

            query = query.then_order_by(
                db_utils::st_distance_4326(
                    // It's ok to assume_not_null here because the filter alread exclude null location records
                    handyman::location.assume_not_null(),
                    point,
                )
                .asc(),
            );
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
    pub distance_within: Option<DistanceWithinFilter>,
}

#[derive(Debug)]
pub struct DistanceWithinFilter {
    pub lon: f64,
    pub lat: f64,
    pub within_meters: f64,
}

impl DistanceWithinFilter {
    pub fn validate(self) -> Result<Self> {
        const MAX_LON: f64 = 180.0;
        const MIN_LON: f64 = -180.0;
        const MAX_LAT: f64 = 90.0;
        const MIN_LAT: f64 = -90.0;

        if self.lon < MIN_LON || self.lon > MAX_LON {
            return Err(Error::invalid_argument_with(
                "Invalid location: Longitude must be between -180.0 and 180.0.",
                Some(BadRequest {
                    field_violations: vec![FieldViolation {
                        field: "location".into(),
                        description: "INVALID_LON".into(),
                    }],
                }),
            ));
        }

        if self.lat < MIN_LAT || self.lat > MAX_LAT {
            return Err(Error::invalid_argument_with(
                "Invalid location: Latitude must be between -90.0 and 90.0.",
                Some(BadRequest {
                    field_violations: vec![FieldViolation {
                        field: "location".into(),
                        description: "INVALID_LAT".into(),
                    }],
                }),
            ));
        }

        Ok(self)
    }
}
