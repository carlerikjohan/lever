// @generated automatically by Diesel CLI.

diesel::table! {
    features (id) {
        id -> Text,
        name -> Text,
        enabled -> Bool,
    }
}
