use seed::prelude::*;
use crate::PageName;
use diet_database::Tabular;
use serde::{Serialize, Deserialize};
use convert_case::{Casing, Case};

const API_URL: &'static str = "http://localhost:8000";

#[derive(Debug, Clone, Copy)]
pub enum ApiCall {
    Bowel,
    Store,
    GroceryTrip,
}

impl ApiCall {
    fn lower(&self) -> String {
        format!("{:?}", self).to_case(Case::Snake)
    }

    pub async fn get<T: 'static + for<'de> Deserialize<'de>>(&self) -> fetch::Result<T> {
        fetch(format!("{}/{}", API_URL, self.lower())).await?.json().await
    }

    pub async fn post<NEW: Serialize>(&self, item: NEW) -> fetch::Result<Response> {
        fetch::Request::new(format!("{}/{}", API_URL, self.lower()))
            .method(Method::Post)
            .json(&item)?
            .fetch()
            .await
    }

    pub async fn delete<ITEM: Serialize>(&self, item: ITEM) -> fetch::Result<Response> {
        fetch::Request::new(format!("{}/{}", API_URL, self.lower()))
            .method(Method::Delete)
            .json(&item)?
            .fetch()
            .await
    }
}
/*
pub mod bowel {
    use super::*;
    use diet_database::bowel::*;

    pub async fn get() -> fetch::Result<Vec<Bowel>> {
        fetch(format!("{}/bowel", API_URL)).await?.json().await
    }

    pub async fn post(bowel: NewBowel) -> fetch::Result<Response> {
        fetch::Request::new(format!("{}/bowel", API_URL))
            .method(Method::Post)
            .json(&bowel)?
            .fetch()
            .await
    }

    pub async fn delete(bowel: Bowel) -> fetch::Result<Response> {
        fetch::Request::new(format!("{}/bowel", API_URL))
            .method(Method::Delete)
            .json(&bowel)?
            .fetch()
            .await
    }
}

pub mod store {
    use super::*;
    use diet_database::store::*;

    pub async fn get() -> fetch::Result<Vec<Store>> {
        fetch(format!("{}/store", API_URL)).await?.json().await
    }

    pub async fn post(store: NewStore) -> fetch::Result<Response> {
        fetch::Request::new(format!("{}/store", API_URL))
            .method(Method::Post)
            .json(&store)?
            .fetch()
            .await
    }

    pub async fn delete(store: Store) -> fetch::Result<Response> {
        fetch::Request::new(format!("{}/store", API_URL))
            .method(Method::Delete)
            .json(&store)?
            .fetch()
            .await
    }
}
*/
