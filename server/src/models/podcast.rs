use podcast_api::Client;
use salvo::async_trait;
use serde_json::json;
use tp_models::podcast::{Podcast, BestPodcasts};

use crate::listennotes;

#[async_trait]
pub trait PodcastQuery {
    async fn fetch_by_id(id: &str) -> Option<Podcast>;
}

#[async_trait]
impl PodcastQuery for Podcast {
    async fn fetch_by_id(id: &str) -> Option<Podcast> {
        let api_key = listennotes::api_key();
        let client = Client::new(api_key);
        let res = client.fetch_podcast_by_id(id, &json!({})).await.ok()?;
        let value = res.json().await.ok()?;
        let data = serde_json::from_value::<Podcast>(value).ok()?;
        Some(data)
    }
}

#[async_trait]
pub trait BestPodcastsQuery {
    async fn get_recommend() -> Option<BestPodcasts>;
}

#[async_trait]
impl BestPodcastsQuery for BestPodcasts {
    async fn get_recommend() -> Option<BestPodcasts> {
        let api_key = listennotes::api_key();
        let client = Client::new(api_key);
        let res = client.fetch_best_podcasts(&json!({})).await.ok()?;
        let info = res.json().await.ok()?;
        Some(serde_json::from_value::<BestPodcasts>(info).ok()?)
    }
}
