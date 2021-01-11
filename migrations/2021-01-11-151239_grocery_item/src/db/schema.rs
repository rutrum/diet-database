table! {
    bowel (id) {
        id -> Integer,
        date -> Date,
        time -> Nullable<Time>,
        scale -> Tinyint,
    }
}

table! {
    grocery_item (id) {
        id -> Integer,
        trip_id -> Integer,
        name -> Varchar,
        amount -> Nullable<Float>,
        measure -> Nullable<Varchar>,
    }
}

table! {
    grocery_trip (id) {
        id -> Integer,
        date -> Date,
        time -> Nullable<Time>,
        store_id -> Integer,
    }
}

table! {
    metric (id) {
        id -> Integer,
        date -> Date,
        time -> Nullable<Time>,
        body_fat -> Nullable<Float>,
        gut_circum -> Nullable<Float>,
        waist_circum -> Nullable<Float>,
        chest_circum -> Nullable<Float>,
        thigh_circum -> Nullable<Float>,
    }
}

table! {
    store (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    weight (id) {
        id -> Integer,
        date -> Date,
        time -> Nullable<Time>,
        value -> Float,
    }
}

joinable!(grocery_item -> grocery_trip (trip_id));
joinable!(grocery_trip -> store (store_id));

allow_tables_to_appear_in_same_query!(
    bowel,
    grocery_item,
    grocery_trip,
    metric,
    store,
    weight,
);
