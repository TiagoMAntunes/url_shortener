// @generated automatically by Diesel CLI.

diesel::table! {
    urls (shortened) {
        shortened -> Varchar,
        original -> Varchar,
    }
}
