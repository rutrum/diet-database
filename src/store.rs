#[cfg(feature = "database")]
use crate::schema::store;

use serde::{Serialize, Deserialize};

use crate::Tabular;

#[cfg_attr(
    feature = "database",
    derive(Insertable),
    table_name = "store",
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewStore {
    pub name: String,
}

#[cfg_attr(
    feature = "database",
    derive(Queryable),
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: i32,
    pub name: String,
}

impl Tabular for Vec<Store> {
    fn headers(&self) -> Vec<String> {
        vec!["Name".to_string()]
    }

    fn matrix(&self) -> Vec<Vec<String>> {
        self.iter().map(|store| vec![store.name.clone()]).collect::<Vec<Vec<String>>>()
    }
}
