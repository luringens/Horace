table! {
    reminders (id) {
        id -> Int4,
        user_id -> Bpchar,
        remind_time -> Date,
        message -> Nullable<Varchar>,
    }
}

table! {
    statistics (guild_id, user_id, post_time) {
        guild_id -> Bpchar,
        user_id -> Bpchar,
        post_time -> Date,
        messages -> Int4,
        words -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    reminders,
    statistics,
);
