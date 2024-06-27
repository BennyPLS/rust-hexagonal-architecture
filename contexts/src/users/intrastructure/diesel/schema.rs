// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        password -> Text,
        email -> Text,
    }
}
