#[cfg(feature = "database")]
use crate::schema::grocery_trip;

use chrono::naive::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::Tabular;

#[cfg_attr(feature = "database", derive(Insertable), table_name = "grocery_trip")]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NewGroceryTrip {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub store_id: i32,
}

#[cfg_attr(feature = "database", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroceryTrip {
    pub id: i32,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub store_name: String,
}

impl Tabular for Vec<GroceryTrip> {
    fn headers(&self) -> Vec<String> {
        let v = vec!["Date", "Time", "Scale"];
        v.iter().map(|x| x.to_string()).collect()
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|trip| {
                let time = match trip.time {
                    None => String::new(),
                    Some(t) => t.format("%-I:%M %p").to_string(),
                };
                vec![
                    trip.date.format("%b %d %Y").to_string(),
                    time,
                    trip.store_name.to_string(),
                ]
            })
            .collect::<Vec<Vec<String>>>()
    }
}
