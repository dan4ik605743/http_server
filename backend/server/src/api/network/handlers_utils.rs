use anyhow::anyhow;

use axum::response::Response;
use axum_extra::extract::CookieJar;
use hyper::StatusCode;

pub type PostResponse<T> = Result<T, AppError>;
pub type ResponseValue = Response<String>;
pub type CookieValue = CookieJar;

// AppErrorHandling
pub struct AppError(pub (anyhow::Error, Option<StatusCode>));

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            {
                tracing::error!("{}", self.0 .0);
                self.0 .1.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            },
            format!("Something went wrong: {}", self.0 .0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self((err.into(), Some(StatusCode::INTERNAL_SERVER_ERROR)))
    }
}

pub fn send_err<T>(bad_code: StatusCode) -> PostResponse<T> {
    Err(AppError((anyhow!(bad_code), Some(bad_code))))
}

pub fn send_response_ok() -> PostResponse<ResponseValue> {
    Ok(Response::new("OK".to_owned()))
}

// pub type JsonValue = Json<Value>;

// // Мейби сделать Option аргументы чтобы можно было возвращать просто { "message": "OK" }
// pub fn send_json_response_ok<T: std::fmt::Display>(
//     field: Vec<&str>,
//     field_data: Vec<T>,
// ) -> PostResponse<JsonValue> {
//     if field.len() != field_data.len() {
//         return Err(AppError((
//             anyhow!("Number of fields and data must be the same"),
//             Some(StatusCode::INTERNAL_SERVER_ERROR),
//         )));
//     }

//     let mut json_object = Map::new();
//     json_object.insert("message".to_string(), json!("OK"));

//     field
//         .iter()
//         .zip(field_data)
//         .for_each(|(&field_name, field_value)| {
//             json_object.insert(field_name.to_string(), json!(field_value.to_string()));
//         });

//     Ok(Json(serde_json::from_str(&serde_json::to_string(
//         &json_object,
//     )?)?))
// }
