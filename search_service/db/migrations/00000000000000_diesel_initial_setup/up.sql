-- Extension for creating tsvector without Vietnamese accent
CREATE EXTENSION unaccent;

CREATE FUNCTION array_deduplicate(anyarray)
RETURNS anyarray AS $$
  SELECT ARRAY(SELECT DISTINCT unnest($1))
$$ LANGUAGE SQL IMMUTABLE;
