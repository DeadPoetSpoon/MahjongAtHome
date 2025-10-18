#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct UserLoginInfo {
    pub username: String,
    pub password: String,
}
