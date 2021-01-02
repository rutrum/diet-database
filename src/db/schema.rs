table! {
    bowel (id) {
        id -> Integer,
        date -> Date,
        time -> Nullable<Time>,
        scale -> Tinyint,
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
        weight -> Nullable<Float>,
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

joinable!(grocery_trip -> store (store_id));

allow_tables_to_appear_in_same_query!(bowel, grocery_trip, metric, store,);
