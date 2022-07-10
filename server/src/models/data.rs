use podcast_api::Client;
use salvo::async_trait;
use serde_json::json;
use tp_models::data::SearchInfo;

use crate::listennotes;

#[async_trait]
pub trait SearchInfoQuery {
    async fn search_episode(q: &str) -> SearchInfo;
}

#[async_trait]
impl SearchInfoQuery for SearchInfo {
    async fn search_episode(q: &str) -> SearchInfo {
        let api_key = listennotes::api_key();
        let client = Client::new(api_key);

        let options = json!({
            "q": q,
            "sort_by_date": "0",
            "type": "episode",
            "offset": "0",
            "only_in": "title,description",
            "safe_mode": "0",
        });

        match client.search(&options).await {
            Ok(response) => {
                let data = response.json().await;
                if let Ok(body) = data {
                    let res = serde_json::from_value::<SearchInfo>(body);
                    // println!("{:#?}", res);
                    return res.unwrap_or_default();
                }
                return SearchInfo::default();
            }
            Err(_) => {
                return SearchInfo::default();
            }
        }
    }
}
