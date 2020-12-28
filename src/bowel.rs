use crate::schema::bowel;
use chrono::naive::{NaiveTime, NaiveDate};

#[derive(Insertable)]
#[table_name = "bowel"]
pub struct NewBowel {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub scale: i8,
}
