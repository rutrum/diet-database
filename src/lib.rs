#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "database")]
pub mod db;

#[cfg(feature = "database")]
pub use db::schema;

pub mod bowel;
