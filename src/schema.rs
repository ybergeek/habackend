table! {
    conditions (time, device_id) {
        time -> Timestamptz,
        device_id -> Text,
        temperature -> Nullable<Numeric>,
        humidity -> Nullable<Numeric>,
    }
}

table! {
    locations (device_id) {
        device_id -> Text,
        location -> Nullable<Text>,
        environment -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(conditions -> locations (device_id));

allow_tables_to_appear_in_same_query!(
    conditions,
    locations,
    users,
);
