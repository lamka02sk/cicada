table! {
    auth_attempts (id) {
        id -> Int4,
        uuid -> Uuid,
        user_id -> Int4,
        user_agent -> Varchar,
        ip_address -> Inet,
        created_at -> Timestamp,
    }
}

table! {
    auth_login (id) {
        id -> Int4,
        uuid -> Uuid,
        user_id -> Int4,
        secret -> Varchar,
        token -> Varchar,
        user_agent -> Varchar,
        ip_address -> Inet,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        token -> Varchar,
        admin -> Bool,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(auth_attempts -> users (user_id));
joinable!(auth_login -> users (user_id));

allow_tables_to_appear_in_same_query!(
    auth_attempts,
    auth_login,
    users,
);
