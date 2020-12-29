use seed::prelude::*;

const API_URL: &'static str = "http://localhost:8000";

pub mod bowel {
    use super::*;
    use diet_database::bowel::{Bowel, NewBowel};

    pub async fn get() -> fetch::Result<Vec<Bowel>> {
        fetch(format!("{}/bowel", API_URL))
            .await?
            .json()
            .await
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
