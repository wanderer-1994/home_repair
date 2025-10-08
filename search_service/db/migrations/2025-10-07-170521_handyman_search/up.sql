-- Search table for handyman. Currently support search by name and skills.
-- In future, we will support search by ranking.

CREATE TABLE handyman (
    handyman_id BIGINT PRIMARY KEY,
    full_name TEXT,
    skills TEXT[],
    search_vector TSVECTOR
);

CREATE INDEX skills_gin_idx ON handyman USING GIN (skills);

CREATE INDEX name_search_idx ON handyman USING GIN (search_vector);

CREATE FUNCTION update_handyman_search_vector() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.full_name IS NULL THEN
        NEW.search_vector = NULL;
        RETURN NEW;
    END IF;

    NEW.search_vector = to_tsvector('simple', unaccent('unaccent', NEW.full_name));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_handyman_search
    BEFORE INSERT OR UPDATE OF full_name ON handyman
        FOR EACH ROW EXECUTE FUNCTION update_handyman_search_vector();

INSERT INTO handyman (handyman_id, full_name) VALUES (
    1, 'Huy'
);

SELECT ARRAY_REMOVE(ARRAY[]::TEXT[], NULL);
SELECT ARRAY_DEDUPLICATE(ARRAY['a', 'b']::TEXT[]);
