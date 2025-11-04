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

#[derive(Debug, Deserialize, Serialize)]
pub struct Meaning {
    pub meaning: String,
    pub primary: bool,
    pub accepted_answer: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reading {
    pub reading: String,
    pub primary: bool,
    pub accepted_answer: bool,
    //wip - this wasn't here when tested
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectData {
    pub characters: Option<String>,
    pub meanings: Vec<Meaning>,
    pub readings: Option<Vec<Reading>>,
    pub level: u8,
    pub slug: String,
    pub component_subject_ids: Option<Vec<u32>>,
    pub amalgamation_subject_ids: Option<Vec<u32>>,
    pub visually_similar_subject_ids: Option<Vec<u32>>,
    pub meaning_mnemonic: Option<String>,
    pub meaning_hint: Option<String>,
    pub reading_mnemonic: Option<String>,
    pub reading_hint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subject {
    pub id: u32,
    pub object: String,
    pub data: SubjectData
}