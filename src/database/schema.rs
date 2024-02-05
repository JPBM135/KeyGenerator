// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int8,
        #[max_length = 255]
        email -> Varchar,
        public_key -> Bytea,
        public_key_fingerprint -> Bytea,
        created_at -> Timestamp,
    }
}
