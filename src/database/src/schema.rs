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
