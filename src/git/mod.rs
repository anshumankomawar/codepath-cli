use std::fs;

use jsonwebtoken::EncodingKey;
use log::debug;
use octocrab::{auth::AppAuth, models::AppId};

const APP_ID: u64 = 1038259;
const GITHUB_PRIVATE_KEY_PATH: &str = "/Users/Ujwal/Downloads/codepath-tutorial.pem";

// TODO: Create struct to encapsulate Git client API

pub struct GitClient {
    pub token: String,
}

impl GitClient {
    pub fn new() -> GitClient {
        let rsa_key =
            fs::read_to_string(GITHUB_PRIVATE_KEY_PATH).expect("Failed to read RSA private key");
        let key =
            EncodingKey::from_rsa_pem(rsa_key.as_ref()).expect("Failed to parse RSA private key");
        let app_auth = AppAuth {
            app_id: AppId::from(APP_ID),
            key,
        };
        let token = app_auth
            .generate_bearer_token()
            .expect("Failed to generate bearer token");

        debug!("Found token {}", token);
        GitClient { token }
    }
}
