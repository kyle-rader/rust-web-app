// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        display_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_salt -> Varchar,
        password_hash -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
