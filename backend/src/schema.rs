// @generated automatically by Diesel CLI.


diesel::table! {
    settings (id) {
        id -> Text,
        dark_mode -> Bool,
        slide_interval -> Integer,
    }
}
