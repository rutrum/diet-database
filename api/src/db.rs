use diesel::{prelude::*, MysqlConnection};
use diet_database::db::schema;
use diet_database::bowel::{NewBowel, Bowel};

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub fn create_connection() -> MysqlConnection {
    let database_url: String = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Cannot connect to database at {}", database_url))
}

pub fn insert_bowel(conn: &MysqlConnection, bowel: NewBowel) -> Result<usize> {
    diesel::insert_into(schema::bowel::table)
        .values(&bowel)
        .execute(conn)
}

pub fn get_bowels(conn: &MysqlConnection) -> Result<Vec<Bowel>> {
    schema::bowel::table
        .load::<Bowel>(conn)
}
