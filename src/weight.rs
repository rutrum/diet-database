#[cfg(feature = "database")]
use crate::schema::weight;

use chrono::naive::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::Tabular;

#[cfg_attr(feature = "database", derive(Insertable), table_name = "weight")]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NewWeight {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub value: f32,
}

#[cfg_attr(feature = "database", derive(Queryable))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Weight {
    pub id: i32,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub value: f32,
}

impl Tabular for Vec<Weight> {
    fn headers(&self) -> Vec<String> {
        let v = vec![
            "Date",
            "Time",
            "Weight",
        ];
        v.iter().map(|x| x.to_string()).collect()
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|item| {
                let time = match item.time {
                    None => String::new(),
                    Some(t) => t.format("%-I:%M %p").to_string(),
                };
                vec![
                    item.date.format("%b %d %Y").to_string(),
                    time,
                    item.value.to_string(),
                ]
            })
            .collect::<Vec<Vec<String>>>()
    }
}
