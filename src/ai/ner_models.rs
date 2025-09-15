use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]

pub struct Entity {
    pub end: usize,
    pub start: usize,
    pub score: f32,
    pub text: String,
    pub label: String
}

#[derive(Debug, Serialize, Deserialize)]

pub struct NERResult {
    pub id: String,
    pub entities: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize)]

pub struct NERRequestResult {
    pub results: Vec<NERResult>
}