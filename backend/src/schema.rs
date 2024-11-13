diesel::table! {
    slides (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
    }
}