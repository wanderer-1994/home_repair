-- Create schedule tables and customer_task_request table

CREATE TABLE schedule (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Map to rust enum ScheduleType
    schedule_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),

    UNIQUE(id, schedule_type)
);

CREATE TABLE schedule_fixed_time (
    id BIGINT PRIMARY KEY REFERENCES schedule(id) ON DELETE CASCADE,
    schedule_type TEXT NOT NULL DEFAULT 'FIXED_TIME' CHECK (schedule_type = 'FIXED_TIME'),

    time TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),

    FOREIGN KEY (id, schedule_type) REFERENCES schedule(id, schedule_type) ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('schedule_fixed_time');

CREATE TABLE schedule_daily_recurrence (
    id BIGINT PRIMARY KEY REFERENCES schedule(id) ON DELETE CASCADE,
    schedule_type TEXT NOT NULL DEFAULT 'DAILY_RECURRENCE' CHECK (schedule_type = 'DAILY_RECURRENCE'),

    times TIME[] NOT NULL CHECK ((array_upper(times, 1) IS NOT NULL) AND (NOT array_contains_null(times))),
    updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),

    FOREIGN KEY (id, schedule_type) REFERENCES schedule(id, schedule_type) ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('schedule_daily_recurrence');

CREATE SEQUENCE schedule_weekly_recurrence_seq;

CREATE TABLE schedule_weekly_recurrence (
    id BIGINT PRIMARY KEY DEFAULT xtea(
        NEXTVAL('schedule_weekly_recurrence_seq'),
        BYTEA '\x87d6a207e4687e17245ea98486bca8a0',
        TRUE
    ),
    schedule_id BIGINT NOT NULL REFERENCES schedule(id) ON DELETE CASCADE,
    schedule_type TEXT NOT NULL DEFAULT 'WEEKLY_RECURRENCE' CHECK (schedule_type = 'WEEKLY_RECURRENCE'),

    -- Map to rust enum Weekday
    weekday TEXT NOT NULL,
    times TIME[] NOT NULL CHECK ((array_upper(times, 1) IS NOT NULL) AND (NOT array_contains_null(times))),
    updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),

    FOREIGN KEY (schedule_id, schedule_type) REFERENCES schedule(id, schedule_type) ON DELETE CASCADE
);

ALTER SEQUENCE schedule_weekly_recurrence_seq OWNED BY schedule_weekly_recurrence.id;

SELECT diesel_manage_updated_at('schedule_weekly_recurrence');

CREATE SEQUENCE customer_task_request_seq;

CREATE TABLE customer_task_request (
    id BIGINT PRIMARY KEY DEFAULT xtea(
        NEXTVAL('customer_task_request_seq'),
        BYTEA '\x646c33a2bad619566f918b78c06de4c5',
        TRUE
    ),

    customer_id BIGINT NOT NULL,
    -- Map to rust enum ServiceLayer2.
    service TEXT NOT NULL,
    title TEXT NOT NULL,
    note TEXT,

    schedule BIGINT NOT NULL REFERENCES schedule(id),

    created_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC'),
    updated_at TIMESTAMP NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC')
);

ALTER SEQUENCE customer_task_request_seq OWNED BY customer_task_request.id;

SELECT diesel_manage_updated_at('customer_task_request');
