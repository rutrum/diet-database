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

#[cfg(feature = "database")]
use diesel::types::*;

#[cfg_attr(feature = "database", derive(Queryable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroceryItem {
    pub id: i32,
    pub trip_desc: String,
    pub name: String,
    pub amount: Option<f32>,
    pub measure: Option<String>,
}

impl Tabular for Vec<GroceryItem> {
    fn headers(&self) -> Vec<String> {
        let v = vec!["Trip", "Name", "Amount"];
        v.iter().map(|x| x.to_string()).collect()
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|item| {
                let amount = match &item.amount {
                    None => String::new(),
                    Some(val) => match &item.measure {
                        None => format!("{}", val),
                        Some(msr) => format!("{} {}", val, msr),
                    }
                };
                vec![
                    item.trip_desc.clone(),
                    item.name.clone(),
                    amount,
                ]
            })
            .collect::<Vec<Vec<String>>>()
    }
}
