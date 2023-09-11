use anyhow::anyhow;
use axum::{response::Response, Json};
use hyper::StatusCode;
use serde_json::{json, Map};

use super::result::{AppError, PostJsonResult, PostResponseResult};

pub fn return_response_error(bad_code: StatusCode) -> PostResponseResult {
    Ok(Response::builder().status(bad_code).body(String::new())?)
}

pub fn return_response_ok() -> PostResponseResult {
    Ok(Response::new("OK".to_owned()))
}

pub fn return_json_error(bad_code: &str) -> PostJsonResult {
    Ok(Json(serde_json::from_str(bad_code)?))
}

// Мейби сделать Option аргументы чтобы можно было возвращать просто { "message": "OK" }
pub fn return_json_ok<T: std::fmt::Display>(
    field: Vec<&str>,
    field_data: Vec<T>,
) -> PostJsonResult {
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
