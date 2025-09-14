-- Database model for sign up customer and handymand

CREATE TABLE customer_account (
  id BIGINT PRIMARY KEY,
  phone_number TEXT NOT NULL,
  password_hash TEXT NOT NULL
);

-- Spitted from customer_account table because
-- during registration profile info does not exist.
CREATE TABLE customer_profile (
  -- Givent that if we ever do database sharding, profile and
  -- account always come together, so referencing is enabled here.
  customer_account_id BIGINT PRIMARY KEY REFERENCES customer_account(id) ON DELETE CASCADE,
  mick_name TEXT NOT NULL
);

CREATE TABLE handyman_account (
  id BIGINT PRIMARY KEY,
  phone_number TEXT NOT NULL,
  password_hash TEXT NOT NULL
);

CREATE TABLE handyman_profile (
  handyman_id BIGINT PRIMARY KEY REFERENCES handyman_account(id) ON DELETE CASCADE,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL
);
