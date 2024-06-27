diesel::table! {
    users {
        id -> Uuid,
        name -> Text,
        password -> Text,
        email -> Text,
    }
}