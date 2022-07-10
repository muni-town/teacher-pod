use serde::{Deserialize, Serialize};

use crate::podcast::Episode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchInfo {
    pub took: f64,
    pub count: usize,
    pub total: usize,
    pub result: Vec<Episode>,
    pub next_offset: usize,
}