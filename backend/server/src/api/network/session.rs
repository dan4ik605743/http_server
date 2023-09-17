use axum_extra::extract::cookie::{Cookie, CookieJar};
// use time::Duration;

pub async fn create_session(jar: CookieJar, username: String) -> CookieJar {
    jar.add(
        Cookie::build(username, "s")
            // .max_age(Duration::hours(1))
            .secure(true)
            .http_only(true)
            .finish(),
    )
}

pub async fn verification_session() {}
