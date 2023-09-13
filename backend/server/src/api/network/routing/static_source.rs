#[non_exhaustive]
pub struct StaticSource;

impl StaticSource {
    pub const SOURCE_DIR: &str = "../frontend";
    pub const ERROR_PAGE: &str = "../frontend/error.html";
    pub const LOGIN_PAGE: &str = "../frontend/login.html";
    pub const REGISTER_PAGE: &str = "../frontend/register.html";
    pub const USER_PAGE: &str = "../frontend/user.html";
}
