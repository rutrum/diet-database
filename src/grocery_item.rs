#[cfg(feature = "database")]
use crate::schema::grocery_item;

use chrono::naive::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::Tabular;

#[cfg_attr(feature = "database", derive(Insertable), table_name = "grocery_item")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewGroceryItem {
    pub trip_id: i32,
    pub name: String,
    pub amount: Option<f32>,
    pub measure: Option<String>,
}

#[cfg_attr(feature = "database", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroceryItem {
    pub id: i32,
    pub trip_desc: i32,
    pub name: String,
    pub amount: Option<f32>,
    pub measure: Option<String>,
}

impl Tabular for Vec<GroceryItem> {
    fn headers(&self) -> Vec<String> {
        let v = vec!["Name", "", "Scale"];
        v.iter().map(|x| x.to_string()).collect()
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|trip| {
                vec![
                    trip.date.format("%b %d %Y").to_string(),
                    time,
                    trip.store_name.to_string(),
                ]
            })
            .collect::<Vec<Vec<String>>>()
    }
}
