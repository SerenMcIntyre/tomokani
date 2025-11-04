use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Learning {
    pub available_at: String,
    pub subject_ids: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Summary {
    pub lessons: Vec<Learning>,
    pub next_reviews_at: String,
    pub reviews: Vec<Learning>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> where T: Serialize {
    pub object: String,
    pub data: T,
}
