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
    store (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(grocery_trip -> store (store_id));

allow_tables_to_appear_in_same_query!(
    bowel,
    grocery_trip,
    store,
);
