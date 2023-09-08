use axum::{http::Response, Json};

use db::{JsonUser, Pool};

#[non_exhaustive]
pub struct FrontendPages;

impl FrontendPages {
    pub const INDEX: &str = include_str!("../../assets/front/index.html");
    pub const REGISTER: &str = include_str!("../../assets/front/register.html");
    pub const LOGIN: &str = include_str!("../../assets/front/login.html");
}

pub async fn get_html_page(html_content: &str) -> Response<String> {
    Response::builder()
        .header("Content-Type", "text/html")
        .body(html_content.to_string())
        .unwrap()
}

pub async fn register(Json(data): Json<JsonUser>, conn: Pool) -> Response<String> {
    let conn = &mut conn.get().unwrap();

    match db::create_user(conn, &data.username, &data.password) {
        Ok(_) => {
            println!("{:#?}", data);
            tracing::info!("Added user: '{}' to database", data.username);
            Response::new("OK".to_owned())
        }
        Err(e) if e.to_string().contains("UNIQUE") => {
            tracing::error!(
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

pub async fn login(Json(data): Json<JsonUser>, conn: Pool) -> Response<String> {
    let conn = &mut conn.get().unwrap();

    Response::new("OK".to_owned())
}
