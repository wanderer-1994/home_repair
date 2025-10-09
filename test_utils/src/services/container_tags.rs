//! Centralized place defining all container tags for unit test
//! which should match with those that is used in other environments (e.g. PROD).

// pub const PG_CONTAINER_TAG: &str = "17.6-alpine";

/// Use custom postgres:custom_postgis image build locally
pub const PG_CONTAINER_TAG: &str = "custom_postgis";
