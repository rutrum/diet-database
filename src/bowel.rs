#[cfg(feature = "database")]
use crate::schema::bowel;

use chrono::naive::{NaiveTime, NaiveDate};
use serde::{Serialize, Deserialize};

#[cfg_attr(
    feature = "database",
    derive(Insertable),
    table_name = "bowel",
)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NewBowel {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub scale: i8,
}

#[cfg_attr(
    feature = "database",
    derive(Queryable),
)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Bowel {
    pub id: i32,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub scale: i8,
}
