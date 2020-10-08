#[derive(Serialize, Default)]
pub struct RespApiKey {
    pub key: String,
    #[serde(rename(serialize = "loggedIn"))]
    pub logged_in: bool,
}

impl RespApiKey {
    pub fn new_logged_in(key: String) -> Self {
        RespApiKey {
            key,
            logged_in: true,
        }
    }
}
