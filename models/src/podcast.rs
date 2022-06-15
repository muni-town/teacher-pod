use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Podcast {
    pub id: String,
    pub r#type: String,
    pub image: String,
    pub title: String,
    pub country: String,
    pub language: String,
    pub thumbnail: String,
    pub is_claimed: bool,
    pub description: String,
    pub total_episodes: i32,
    pub audio_length_sec: i32,
    pub explicit_content: bool,
    pub latest_episode_id: String,
    pub listen_score_global_rank: String,
}

// impl Podcast {
//     pub async fn fetch_by_id(id: &str) -> Option<Podcast> {
//         let api_key = listennotes::api_key();
//         let client = Client::new(api_key);
//         let res = client.fetch_podcast_by_id(id, &json!({})).await.ok()?;
//         let value = res.json().await.ok()?;
//         let data = serde_json::from_value::<Podcast>(value).ok()?;
//         Some(data)
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct BestPodcasts {
    pub id: i32,
    pub name: String,
    pub total: i32,
    pub has_next: bool,
    pub podcasts: Vec<Podcast>,
    pub parent_id: i32,
    pub page_number: i32,
    pub has_previous: bool,
    pub next_page_number: i32,
    pub previous_page_number: i32,
}

// impl BestPodcasts {
//     pub async fn get_recommend() -> Option<BestPodcasts> {
//         let api_key = listennotes::api_key();
//         let client = Client::new(api_key);
//         let res = client.fetch_best_podcasts(&json!({})).await.ok()?;
//         let info = res.json().await.ok()?;
//         println!("{:?}", serde_json::from_value::<BestPodcasts>(info.clone()));
//         Some(serde_json::from_value::<BestPodcasts>(info).ok()?)
//     }
// }