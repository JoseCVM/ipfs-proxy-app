table! {
    keys (api_key) {
        api_key -> Text,
        expires_in -> Int4,
        is_enabled -> Bool,
        userid -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    requests (id) {
        id -> Int4,
        api_key -> Text,
        api_call -> Text,
        request_size -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        created_at -> Timestamp,
    }
}

joinable!(keys -> users (userid));
joinable!(requests -> keys (api_key));

allow_tables_to_appear_in_same_query!(
    keys,
    requests,
    users,
);
