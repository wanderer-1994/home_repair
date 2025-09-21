-- Database model for sign up customer and handymand

CREATE SEQUENCE customer_account_id_seq;
CREATE TABLE customer_account (
  -- ```bash
  -- openssl rand -hex 16
  -- ```
  id BIGINT PRIMARY KEY DEFAULT xtea(
    NEXTVAL('customer_account_id_seq'),
    BYTEA '\x7984e3d42db45f229098f7f67b0b8523',
    TRUE
  ),
  phone_number TEXT NOT NULL,
  password_hash TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
  updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);
ALTER SEQUENCE customer_account_id_seq OWNED BY customer_account.id;

-- Spitted from customer_account table because
-- during registration profile info does not exist.
CREATE TABLE customer_profile (
  -- Givent that if we ever do database sharding, profile and
  -- account always come together, so referencing is enabled here.
  customer_id BIGINT PRIMARY KEY REFERENCES customer_account(id) ON DELETE CASCADE,
  nick_name TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
  updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);

CREATE SEQUENCE handyman_account_id_seq;
CREATE TABLE handyman_account (
  id BIGINT PRIMARY KEY DEFAULT xtea(
    NEXTVAL('handyman_account_id_seq'),
    BYTEA '\x9f9f49308d54afb7d6135eb29c812b3d',
    TRUE
  ),
  phone_number TEXT NOT NULL,
  password_hash TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
  updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);
ALTER SEQUENCE handyman_account_id_seq OWNED BY handyman_account.id;

CREATE TABLE handyman_profile (
  handyman_id BIGINT PRIMARY KEY REFERENCES handyman_account(id) ON DELETE CASCADE,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
  updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);
