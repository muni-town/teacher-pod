use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    pub episodes: Vec<Episode>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Episode {
    pub id: String,
    pub link: String,
    pub audio: String,
    pub image: String,
    pub title: String,
    pub thumbnail: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
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