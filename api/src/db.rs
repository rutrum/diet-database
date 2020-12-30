use diesel::{prelude::*, MysqlConnection};
use diet_database::db::schema;

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub fn create_connection() -> MysqlConnection {
    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Cannot connect to database at {}", database_url))
}

pub mod bowel {
    use super::*;
    use diet_database::bowel::*;

    pub fn insert(conn: &MysqlConnection, bowel: NewBowel) -> Result<usize> {
        diesel::insert_into(schema::bowel::table)
            .values(&bowel)
            .execute(conn)
    }

    pub fn select_all(conn: &MysqlConnection) -> Result<Vec<Bowel>> {
        schema::bowel::table.load::<Bowel>(conn)
    }

    pub fn delete(conn: &MysqlConnection, del_bowel: Bowel) -> Result<usize> {
        use schema::bowel::dsl::*;
        diesel::delete(bowel.filter(id.eq(del_bowel.id))).execute(conn)
    }
}

pub mod store {
    use super::*;
    use diet_database::store::*;

    pub fn insert(conn: &MysqlConnection, store: NewStore) -> Result<usize> {
        diesel::insert_into(schema::store::table)
            .values(&store)
            .execute(conn)
    }

    pub fn select_all(conn: &MysqlConnection) -> Result<Vec<Store>> {
        schema::store::table.load::<Store>(conn)
    }

    pub fn delete(conn: &MysqlConnection, del_store: Store) -> Result<usize> {
        use schema::store::dsl::*;
        diesel::delete(store.filter(id.eq(del_store.id))).execute(conn)
    }
}

pub mod grocery_trip {
    use super::*;
    use diet_database::grocery_trip::*;

    pub fn insert(conn: &MysqlConnection, trip: NewGroceryTrip) -> Result<usize> {
        diesel::insert_into(schema::grocery_trip::table)
            .values(&trip)
            .execute(conn)
    }

    pub fn select_all(conn: &MysqlConnection) -> Result<Vec<GroceryTrip>> {
        use schema::grocery_trip::dsl::*;
        use schema::store::{name, self};
        grocery_trip.inner_join(store::table)
            .select((id, date, time, name))
            .load(conn)
    }

    pub fn delete(conn: &MysqlConnection, del_trip: GroceryTrip) -> Result<usize> {
        use schema::grocery_trip::dsl::*;
        diesel::delete(grocery_trip.filter(id.eq(del_trip.id))).execute(conn)
    }
}
