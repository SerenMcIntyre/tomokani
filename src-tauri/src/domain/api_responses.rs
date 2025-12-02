use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseObject {
    pub id: u32,
    pub object: String,
}

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
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterImage {
    pub url: String,
    pub content_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectData {
    pub characters: Option<String>,
    pub character_images: Option<Vec<CharacterImage>>,
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
    #[serde(flatten)]
    pub base: BaseObject,
    pub data: SubjectData
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssignmentData {
    pub subject_id: u32,
    pub srs_stage: u8,
    pub available_at: Option<String>,
    pub hidden: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Assignment {
    #[serde(flatten)]
    pub base: BaseObject,
    pub data: AssignmentData,
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct Subscription {
    pub active: bool,
    pub max_level_granted: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub level: u8,
    pub subscription: Subscription
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReviewData {
    pub subject_id: Option<u32>,
    pub assignment_id: Option<u32>,
    pub incorrect_meaning_answers: u32,
    pub incorrect_reading_answers: u32,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReviewPayload {
    pub review: ReviewData,
}