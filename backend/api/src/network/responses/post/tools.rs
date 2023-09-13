use anyhow::anyhow;
use axum::{response::Response, Json};
use hyper::StatusCode;
use serde_json::{json, Map};

use super::{AppError, PostJsonResponse, PostResponse};

pub fn send_response_error(bad_code: StatusCode) -> PostResponse {
    Ok(Response::builder().status(bad_code).body(String::new())?)
}

pub fn send_response_ok() -> PostResponse {
    Ok(Response::new("OK".to_owned()))
}

pub fn send_json_response_error(bad_code: &str) -> PostJsonResponse {
    Ok(Json(serde_json::from_str(bad_code)?))
}

// Мейби сделать Option аргументы чтобы можно было возвращать просто { "message": "OK" }
pub fn send_json_response_ok<T: std::fmt::Display>(
    field: Vec<&str>,
    field_data: Vec<T>,
) -> PostJsonResponse {
    if field.len() != field_data.len() {
        return Err(AppError(anyhow!(
            "Number of fields and data must be the same"
        )));
    }

    let mut json_object = Map::new();
    json_object.insert("message".to_string(), json!("OK"));

    field
        .iter()
        .zip(field_data)
        .for_each(|(&field_name, field_value)| {
            json_object.insert(field_name.to_string(), json!(field_value.to_string()));
        });

    Ok(Json(serde_json::from_str(&serde_json::to_string(
        &json_object,
    )?)?))
}
