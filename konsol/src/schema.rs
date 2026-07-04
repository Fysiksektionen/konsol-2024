// @generated automatically by Diesel CLI.

diesel::table! {
    settings (id) {
        id -> Integer,
        layout_type -> Text,
        color_mode -> Text,
    }
}

diesel::table! {
    slides (id) {
        id -> Text,
        caption -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        active -> Bool,
        filetype -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(settings, slides, users,);
