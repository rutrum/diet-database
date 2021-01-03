#[cfg(feature = "database")]
use crate::schema::metric;

use chrono::naive::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::Tabular;

#[cfg_attr(feature = "database", derive(Insertable), table_name = "metric")]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NewMetric {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub body_fat: Option<f32>,
    pub gut_circum: Option<f32>,
    pub waist_circum: Option<f32>,
    pub chest_circum: Option<f32>,
    pub thigh_circum: Option<f32>,
}

#[cfg_attr(feature = "database", derive(Queryable))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Metric {
    pub id: i32,
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub body_fat: Option<f32>,
    pub gut_circum: Option<f32>,
    pub waist_circum: Option<f32>,
    pub chest_circum: Option<f32>,
    pub thigh_circum: Option<f32>,
}

impl Tabular for Vec<Metric> {
    fn headers(&self) -> Vec<String> {
        let v = vec![
            "Date",
            "Time",
            "Body Fat %",
            "Gut",
            "Waist",
            "Chest",
            "Thigh",
        ];
        v.iter().map(|x| x.to_string()).collect()
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|metric| {
                let time = match metric.time {
                    None => String::new(),
                    Some(t) => t.format("%-I:%M %p").to_string(),
                };
                vec![
                    metric.date.format("%b %d %Y").to_string(),
                    time,
                    metric.body_fat.map(|x| x.to_string()).unwrap_or_default(),
                    metric.gut_circum.map(|x| x.to_string()).unwrap_or_default(),
                    metric
                        .waist_circum
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                    metric
                        .chest_circum
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                    metric
                        .thigh_circum
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                ]
            })
            .collect::<Vec<Vec<String>>>()
    }
}
