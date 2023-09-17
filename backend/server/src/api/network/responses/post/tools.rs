use anyhow::anyhow;
use axum::{response::Response, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use hyper::StatusCode;
use serde_json::{json, Map};

use super::{AppError, JsonValue, PostResponse, ResponseValue};

pub fn send_response_ok() -> PostResponse<ResponseValue> {
    Ok(Response::new("OK".to_owned()))
}

// Мейби сделать Option аргументы чтобы можно было возвращать просто { "message": "OK" }
pub fn send_json_response_ok<T: std::fmt::Display>(
    field: Vec<&str>,
    field_data: Vec<T>,
) -> PostResponse<JsonValue> {
    if field.len() != field_data.len() {
        return Err(AppError((
            anyhow!("Number of fields and data must be the same"),
            Some(StatusCode::INTERNAL_SERVER_ERROR),
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

pub fn send_err<T>(bad_code: StatusCode) -> PostResponse<T> {
    Err(AppError((anyhow!(bad_code), Some(bad_code))))
}

// pub fn send_cookie_response_ok(
//     cookie: CookieJar,
//     field: String,
//     field_data: String,
// ) -> PostCookieResponse {
//     Ok(cookie.add(Cookie::new(field, field_data)))
// }
