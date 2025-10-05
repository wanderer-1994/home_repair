-- Handyman declare expertise and pricing reference

CREATE TABLE handyman_expertise (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    handyman_id BIGINT NOT NULL,
    -- Map to rust enum `ServiceLayer2`
    service TEXT NOT NULL,
    note TEXT,
    -- Reference pricing in VND
    rate_vnd INT,
    created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);

CREATE UNIQUE INDEX handyman_expertise_handyman_id_service_note_unique
    ON handyman_expertise (handyman_id, service) WHERE (note IS NULL);
CREATE INDEX handyman_expertise_service_idx ON handyman_expertise(service);
