use telegram_oauth::TelegramOauth as Oauth;
pub use telegram_oauth::TelegramOauthError;

use crate::model::TelegramUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramOauth {
    pub hash: String,
    pub id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
    pub auth_date: u64,
}

impl TelegramOauth {
    pub fn check_telegram_authorization(
        self,
    ) -> Result<TelegramUser, (TelegramOauthError, TelegramUser)> {
        let telegram_token = dotenv::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");

        let auth_result = Oauth {
            hash: &self.hash,
            id: self.id,
            username: self.username.as_deref(),
            first_name: self.first_name.as_deref(),
            last_name: self.last_name.as_deref(),
            photo_url: self.photo_url.as_deref(),
            auth_date: self.auth_date,
        }
        .verify(&telegram_token, 86400);

        match auth_result {
            Ok(_) => Ok(self.into_telegram_user()),
            Err(err) => Err((err, self.into_telegram_user())),
        }
    }

    fn into_telegram_user(self) -> TelegramUser {
        let TelegramOauth {
            id,
            username,
            first_name,
            last_name,
            photo_url,
            ..
        } = self;
        TelegramUser {
            id,
            username: username.unwrap_or_default(),
            first_name: first_name.unwrap_or_default(),
            last_name: last_name.unwrap_or_default(),
            photo_url: photo_url.unwrap_or_default(),
        }
    }
}
