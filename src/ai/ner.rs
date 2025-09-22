use reqwest;
use std::collections::{HashMap, HashSet};
use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};
use crate::db::{
    connection::AppState,
    models::{Comment},
    operations::{CommentRepository}
};
use crate::routes::errors::AppError;


#[derive(Debug, Serialize, Deserialize)]
pub struct NERRequest {
    video_id: String,
    labels: Vec<String>,
    threshold: f32
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Annotations(HashMap<String, HashSet<String>>);

impl Annotations {
    fn merge(&mut self, other: Annotations) {
        for (label, values) in other.0 {
            self.0.entry(label).or_default().extend(values);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnnotationObject {
    pub id: String,
    pub annotations: Annotations
}

pub async fn ner_request(ner_request: NERRequest, State(app_state): State<AppState>) -> Result<Vec<Comment>, AppError> {
    let video_request_id = ner_request.video_id.as_str();
    let comments = CommentRepository::get_by_video_id(&app_state.db_pool, video_request_id).await?;

    let content_and_ids = CommentRepository::get_comment_content_and_ids(&comments);

    let payload = json!({
        "comments": content_and_ids,
        "labels": ner_request.labels,
        "threshold": ner_request.threshold
    });

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/ner")
        .json(&payload)
        .send()
        .await.map_err(|e| AppError::AIServerError(e.to_string()))?;

    let ner_results: NERRequestResult = response
        .json::<NERRequestResult>()
        .await
        .map_err(|e| AppError::AIServerError(e.to_string()))?;

    let merged_results = merge_db_json_and_ner_results(&comments, ner_results);

    let updated_comments = CommentRepository::update_annotations(&app_state.db_pool, merged_results).await?;

    Ok(updated_comments)
}

pub fn merge_db_json_and_ner_results(comments: &Vec<Comment>, ner_results:NERRequestResult) -> Vec<AnnotationObject> {
    let mut merged_annotations: Vec<AnnotationObject> = Vec::new();

    let ner_annotations = build_ner_results_as_annotations(ner_results);
    let db_annotations = build_db_json_as_annotations(comments);

    for annotation in ner_annotations {
        let found_comment = db_annotations.iter().find(|comment| comment.id == annotation.id);

        match found_comment {
            None => {
                println!("Can't locate associated comment");
                continue
            },
            Some(found_comment) =>{
                let mut db_comment_annotations = found_comment.annotations.clone();

                db_comment_annotations.merge(annotation.annotations);

                let final_annotation_object = AnnotationObject {
                    id: found_comment.id.clone(),
                    annotations: db_comment_annotations
                };
                merged_annotations.push(final_annotation_object);
            }
        }
    }
    merged_annotations
}
pub fn build_db_json_as_annotations(comments: &Vec<Comment>) -> Vec<AnnotationObject> {
    let mut annotation_objects: Vec<AnnotationObject> = Vec::new();

    for comment in comments {
        if let Some(serde_json::Value::Object(map)) = &comment.annotations {
            let mut annotations = Annotations::default();

            for (label, value) in map {
                let key = label.to_lowercase();

                match value {
                    serde_json::Value::Array(arr) => {
                        for item in arr {
                            let text = match item {
                                serde_json::Value::String(s) => s.clone(),
                                other => other.to_string(),
                            };
                            if !text.is_empty() {
                                annotations
                                    .0
                                    .entry(key.clone())
                                    .or_default()
                                    .insert(text);
                            }
                        }
                    }
                    serde_json::Value::String(s) => {
                        if !s.is_empty() {
                            annotations
                                .0
                                .entry(key.clone())
                                .or_default()
                                .insert(s.clone());
                        }
                    }
                    other => {
                        let text = other.to_string();
                        if !text.is_empty() {
                            annotations
                                .0
                                .entry(key.clone())
                                .or_default()
                                .insert(text);
                        }
                    }
                }
            }

            let annotation_object = AnnotationObject {
                id: comment.comment_id.clone(),
                annotations,
            };

            annotation_objects.push(annotation_object);
        }
    }
    annotation_objects
}

pub fn build_ner_results_as_annotations(ner_results: NERRequestResult) -> Vec<AnnotationObject> {
    let mut annotation_objects: Vec<AnnotationObject> = Vec::new();

    // Track the comment IDs I've handled
    let mut tracking_set:HashSet<String> = HashSet::new();

    for result in ner_results.results {
        let comment_id = result.id;

        let entities = result.entities;

        let mut annotations = Annotations::default();

        for entity in entities {
            annotations.0
                .entry(entity.label.clone())
                .or_insert_with(HashSet::new)
                .insert(entity.text.clone());
        }

        let annotation_object = AnnotationObject {
            id: comment_id.clone(),
            annotations
        };

        let set_contains_id = tracking_set.contains(&comment_id);

        if (!set_contains_id) {
            tracking_set.insert(comment_id.clone());
            annotation_objects.push(annotation_object);
        } else {
           let annotation_object_index = annotation_objects.iter().position(|annotation| annotation.id == comment_id);
            match annotation_object_index {
                Some(index) => {
                    let annotation_object_to_remove = annotation_objects.remove(index);
                    let existing_annotations = annotation_object_to_remove.annotations;
                    let mut new_annotations = annotation_object.annotations;

                    new_annotations.merge(existing_annotations);

                    let new_annotation_object = AnnotationObject{
                        id: comment_id,
                        annotations: new_annotations
                    };
                    annotation_objects.push(new_annotation_object);

                },
                None => {
                    println!("Invalid index. Unable to locate index in annotation_objects vec during NER operation.");
                }
            }
        }
    }
    annotation_objects
}

