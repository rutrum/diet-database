#[cfg(feature = "database")]
use crate::schema::bowel;

use chrono::naive::{NaiveTime, NaiveDate};
use serde::{Serialize, Deserialize};

use crate::Tabular;

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

impl Tabular for Vec<Bowel> {
    fn headers(&self) -> Vec<String> {
        let v = vec!["Date", "Time", "Scale"];
        v.iter().map(|x| x.to_string()).collect()
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter().map(|bowel| {
            let time = match bowel.time {
                None => String::new(), 
                Some(t) => t.format("%-I:%M %p").to_string(),
            };
            vec![
                bowel.id.to_string(),
                bowel.date.format("%b %d %Y").to_string(),
                time,
                bowel.scale.to_string(),
            ]
        }).collect::<Vec<Vec<String>>>()
    }
}
