-- Search table for handyman. Currently support search by name and skills.
-- In future, we will support search by ranking.

CREATE TABLE handyman (
    handyman_id BIGINT PRIMARY KEY,
    full_name TEXT,
    skills TEXT[],
    search_vector TSVECTOR,
    avg_rating_score SMALLINT CHECK (avg_rating_score >= 0 AND avg_rating_score <= 500),
    -- GEOGRAPHY type available by `CREATE EXTENSION postgis;`
    -- Available geography shapes: POINT | LINESTRING | POLYGON | MULTIPOLYGON | MULTILINESTRING | MULTIPOINT | GEOMETRYCOLLECTION
    location GEOGRAPHY(POINT, 4326)
);

CREATE INDEX handyman_skills_gin_idx ON handyman USING GIN (skills);
CREATE INDEX handyman_full_name_idx ON handyman USING GIN (search_vector);
CREATE INDEX handyman_location_idx ON handyman USING GIST (location);

CREATE FUNCTION handyman_update_search_vector() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.full_name IS NULL THEN
        NEW.search_vector = NULL;
        RETURN NEW;
    END IF;

    NEW.search_vector = to_tsvector('simple', unaccent('unaccent', NEW.full_name));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER handyman_search_update
    BEFORE INSERT OR UPDATE OF full_name ON handyman
        FOR EACH ROW EXECUTE FUNCTION handyman_update_search_vector();

-- SELECT ARRAY_REMOVE(ARRAY[]::TEXT[], NULL);
-- SELECT ARRAY_DEDUPLICATE(ARRAY['a', 'b']::TEXT[]);

-- -- Example: Handyman located at Longitude -73.935242, Latitude 40.730610 (New York)
-- INSERT INTO handyman (handyman_id, location) VALUES
-- (2, ST_SetSRID(ST_MakePoint(-73.935242, 40.730610), 4326)::GEOGRAPHY),
-- (3, ST_SetSRID(ST_MakePoint(-73.9866, 40.7580), 4326)::GEOGRAPHY);

-- SELECT
--     handyman_id,
--     ST_Distance(
--         location,
--         ST_SetSRID(ST_MakePoint(-73.9856, 40.7484), 4326)::GEOGRAPHY
--     ) AS distance_meters
-- FROM
--     handyman
-- WHERE
--     -- Filter handyman within 5,000 meters (5 km) of the customer's location
--     ST_DWithin(
--         location,
--         ST_SetSRID(ST_MakePoint(-73.9856, 40.7484), 4326)::GEOGRAPHY,
--         5000  -- Radius in meters (5 km)
--     )
-- ORDER BY
--     distance_meters ASC;

-- SELECT ST_Distance(ST_SetSRID(ST_MakePoint(100.0000, 40.0000), 4326)::GEOGRAPHY, ST_SetSRID(ST_MakePoint(101.0000, 41.0000), 4326)::GEOGRAPHY) AS shortest_distance_in_degrees;
-- SELECT ST_Distance(ST_GeomFromText('POLYGON((0 0, 0 10, 10 10, 10 0, 0 0))', 4326), ST_GeomFromText('POLYGON((20 20, 20 30, 30 30, 30 20, 20 20))', 4326)) AS shortest_distance_in_degrees;
