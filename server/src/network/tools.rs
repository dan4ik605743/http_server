use axum::{http::Response, Json};
use serde_json::{json, Value};

use db::{JsonUser, Pool, UserError};

pub async fn register(Json(data): Json<JsonUser>, conn: Pool) -> Response<String> {
    let conn = &mut conn.get().unwrap();

    match db::create_user(conn, &data.username, &data.password) {
        Ok(_) => {
            tracing::info!("Added user: '{}' to database", data.username);
            Response::new("OK".to_owned())
        }
        Err(e) if e.to_string().contains("UNIQUE") => {
            tracing::warn!(
                "A user with the same name: '{}' already exists",
                data.username,
            );
            Response::builder().status(409).body(String::new()).unwrap()
        }
        Err(e) => {
            tracing::error!("{e}");
            Response::builder().status(500).body(String::new()).unwrap()
        }
    }
}

// pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> Json<Value> {
//     let conn = &mut conn.get().unwrap();

//     match db::verification_user(conn, &data.username, &data.password) {
//         Ok(_) => Json(json!({ "message": "OK", "username": data.username })),
//         Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
//             tracing::warn!("'{}': {e}", data.username);
//             Json(json!({ "error": "Not Found" }))
//         }
//         Err(e) => {
//             tracing::warn!("'{}': {e}", data.username);
//             Json(json!({ "error": "Unauthorized" }))
//         }
//     }
// }

// pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> Response<String> {
//     let conn = &mut conn.get().unwrap();

//     match db::verification_user(conn, &data.username, &data.password) {
//         Ok(_) => Response::new("OK".to_owned()),
//         Err(e) if e.downcast_ref() == Some(&UserError::NotFoundUser) => {
//             tracing::warn!("'{}': {e}", data.username);
//             Response::builder().status(404).body(String::new()).unwrap()
//         }
//         Err(e) => {
//             tracing::warn!("'{}': {e}", data.username);
//             Response::builder().status(401).body(String::new()).unwrap()
//         }
//     }
// }

pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> Json<Value> {
    Json(json!({ "message": "OK", "username": data.username }))
}
