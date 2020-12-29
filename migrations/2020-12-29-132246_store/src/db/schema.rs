table! {
    bowel (id) {
        id -> Integer,
        date -> Date,
        time -> Nullable<Time>,
        scale -> Tinyint,
    }
}

table! {
    store (id) {
        id -> Integer,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    bowel,
    store,
);
