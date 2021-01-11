#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;

use convert_case::{Case, Casing};

#[cfg(feature = "database")]
pub mod db;

#[cfg(feature = "database")]
pub use db::schema;

pub mod bowel;
pub mod grocery_trip;
pub mod grocery_item;
pub mod metric;
pub mod weight;
pub mod store;

pub trait Tabular: std::default::Default {
    fn headers(&self) -> Vec<String>;
    fn matrix(&self) -> Vec<Vec<String>>;
}

#[derive(Debug, Clone, Copy)]
pub enum TableType {
    Bowel,
    GroceryTrip,
    Metric,
    Weight,
    Store,
}

impl TableType {
    pub fn snake_case(&self) -> String {
        format!("{:?}", self).to_case(Case::Snake)
    }
}
