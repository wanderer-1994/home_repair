// @generated automatically by Diesel CLI.

diesel::table! {
    customer_task_request (id) {
        id -> Int8,
        customer_id -> Int8,
        service -> entity_type::ServiceLayer2Mapping,
        title -> Text,
        note -> Nullable<Text>,
        schedule -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    handyman_service (id) {
        id -> Int8,
        handyman_id -> Int8,
        service -> entity_type::ServiceLayer2Mapping,
        note -> Nullable<Text>,
        rate_vnd -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    schedule (id) {
        id -> Int8,
        schedule_type -> entity_type::ScheduleTypeMapping,
        created_at -> Timestamp,
    }
}

diesel::table! {
    schedule_daily_recurrence (id) {
        id -> Int8,
        schedule_type -> entity_type::ScheduleTypeMapping,
        times -> Array<Time>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    schedule_fixed_time (id) {
        id -> Int8,
        schedule_type -> entity_type::ScheduleTypeMapping,
        time -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    schedule_weekly_recurrence (id) {
        id -> Int8,
        schedule_id -> Int8,
        schedule_type -> entity_type::ScheduleTypeMapping,
        weekday -> entity_type::WeekdayMapping,
        times -> Array<Time>,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(customer_task_request -> schedule (schedule));

diesel::allow_tables_to_appear_in_same_query!(
    customer_task_request,
    handyman_service,
    schedule,
    schedule_daily_recurrence,
    schedule_fixed_time,
    schedule_weekly_recurrence,
);
