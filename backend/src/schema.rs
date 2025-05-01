// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        #[max_length = 255]
        date -> Varchar,
        inhalt -> Text,
        percent -> Integer,
    }
}
