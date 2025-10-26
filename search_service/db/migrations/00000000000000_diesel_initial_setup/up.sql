-- Extension for creating tsvector without Vietnamese accent
CREATE EXTENSION unaccent;
CREATE EXTENSION postgis;

CREATE FUNCTION array_deduplicate(anyarray)
RETURNS anyarray AS $$
  SELECT ARRAY(SELECT DISTINCT unnest($1))
$$ LANGUAGE SQL IMMUTABLE;

CREATE FUNCTION array_contains_null(anyarray)
RETURNS BOOLEAN AS $$
  SELECT EXISTS (SELECT 1 FROM unnest($1) AS e WHERE e IS NULL);
$$ LANGUAGE SQL IMMUTABLE;
