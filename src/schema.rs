// @generated automatically by Diesel CLI.

diesel::table! {
    rooms (id) {
        id -> Integer,
        #[max_length = 45]
        type_room -> Varchar,
        #[max_length = 45]
        name -> Varchar,
        #[max_length = 2000]
        users -> Varchar,
    }
}
