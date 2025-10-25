#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct UserLoginInfo {
    pub username: String,
    pub password: String,
    pub token: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub token: String,
    pub role: String,
}
