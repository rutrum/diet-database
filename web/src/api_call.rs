use convert_case::{Case, Casing};
use seed::prelude::*;
use serde::{Deserialize, Serialize};

const API_URL: &str = "http://localhost:8000";

#[derive(Debug, Clone, Copy)]
pub enum ApiCall {
    Bowel,
    Store,
    GroceryTrip,
    Metric,
}

impl ApiCall {
    fn lower(&self) -> String {
        format!("{:?}", self).to_case(Case::Snake)
    }

    pub async fn get<T: 'static + for<'de> Deserialize<'de>>(&self) -> fetch::Result<T> {
        fetch(format!("{}/{}", API_URL, self.lower()))
            .await?
            .json()
            .await
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
