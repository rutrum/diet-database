use crate::schema::bowel;
use chrono::naive::{NaiveTime, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Insertable)]
#[derive(Serialize, Deserialize)]
#[table_name = "bowel"]
pub struct NewBowel {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub scale: i8,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Bowel {
    pub id: i32,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub scale: i8,
}
