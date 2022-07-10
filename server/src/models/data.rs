use podcast_api::Client;
use salvo::async_trait;
use serde_json::json;
use tp_models::data::SearchInfo;

use crate::listennotes;

#[async_trait]
pub trait SearchInfoQuery {
    async fn search_by_str(q: &str) -> SearchInfo;
}

#[async_trait]
impl SearchInfoQuery for SearchInfo {
    async fn search_by_str(q: &str) -> SearchInfo {
        let api_key = listennotes::api_key();
        let client = Client::new(api_key);

        let options = json!({
            "q": "123",
        });

        match client.search(&options).await {
            Ok(_) => {}
            Err(_) => {}
        }

        todo!()
    }
}
