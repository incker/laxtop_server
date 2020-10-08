use std::process::Command;
use std::str;

use crate::base::RespErrors;

#[derive(Debug, Deserialize)]
pub struct FirebaseError {
    pub code: String,
    pub message: String,
}

impl FirebaseError {
    pub fn into_resp_errors(self) -> RespErrors {
        let FirebaseError { code, message } = self;
        let errors = vec![("code".to_string(), code), ("message".to_string(), message)];
        RespErrors { errors }
    }
}

#[derive(Debug, Deserialize)]
pub struct FirebaseUserInfo {
    pub uid: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInDetails {
    #[serde(rename = "firebaseToken")]
    firebase_token: String,
}

impl SignInDetails {
    pub fn get_user_info_by_token(&self) -> Result<FirebaseUserInfo, FirebaseError> {
        let node_file = {
            let mut node_file = dotenv::var("NODE_PATH").expect("NODE_PATH is not set");
            node_file.push_str("/main.js");
            node_file
        };

        let output = Command::new("node")
            .arg(&node_file)
            .arg("--token")
            .arg(&self.firebase_token)
            .output()
            .expect("failed to execute process");

        let node_resp = str::from_utf8(&output.stdout).unwrap().trim();

        serde_json::from_str::<FirebaseUserInfo>(node_resp).map_err(|_| {
            serde_json::from_str::<FirebaseError>(node_resp).unwrap_or(FirebaseError {
                code: "panic".into(),
                message: node_resp.into(),
            })
        })
    }
}
