use keyring::Entry;
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{error::Error, process::Command};
use tokio::time::{sleep, Duration};

const CLIENT_ID: &str = "Ov23liNJRwNr37nSOFCg";
const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const SCOPE: &str = "repo";

#[derive(Debug, Serialize, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

fn open_url(url: &str) {
    match std::env::consts::OS {
        "windows" => Command::new("cmd").args(["/C", "start", url]).spawn(),
        "macos" => Command::new("open").arg(url).spawn(),
        "linux" => Command::new("xdg-open").arg(url).spawn(),
        _ => return,
    }
    .map_err(|e| println!("Failed to open browser: {}", e))
    .ok();
}

async fn get_device_code(client: &Client) -> Result<DeviceCodeResponse, Box<dyn Error>> {
    let response = client
        .post(DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .form(&[("client_id", CLIENT_ID), ("scope", SCOPE)])
        .send()
        .await?;

    let device_code_response = response.json::<DeviceCodeResponse>().await?;
    println!("Please enter the code: {}", device_code_response.user_code);
    println!("Opening browser for authorization...");

    let url = &device_code_response.verification_uri;
    open_url(url);

    Ok(device_code_response)
}

pub fn create_repository_from_template(
    org: &str,
    template_repo: &str,
    new_repo_name: &str,
    user_token: &str,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let url = format!(
        "https://api.github.com/repos/{}/{}/generate",
        org, template_repo
    );

    let request_body = json!({
        "name": new_repo_name,
        "owner": org,
        "private": false,
    });

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "MyApp/1.0".parse().unwrap());

    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", user_token))
        .header("Accept", "application/vnd.github.v3+json")
        .headers(headers)
        .json(&request_body)
        .send()?;

    if response.status().is_success() {
        println!(
            "Repository '{}' created successfully from template '{}'.",
            new_repo_name, template_repo
        );
    } else {
        let error_message = response.text()?;
        eprintln!("Failed to create repository: {}", error_message);
    }

    Ok(())
}

async fn poll_for_access_token(
    client: &Client,
    device_code: String,
    interval: u64,
) -> Result<String, Box<dyn Error>> {
    println!("Waiting for user authorization...");
    loop {
        let response = client
            .post(TOKEN_URL)
            .header("Accept", "application/json")
            .form(&[
                ("client_id", CLIENT_ID),
                ("device_code", &device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?;

        let token_response = response.json::<AccessTokenResponse>().await?;

        if let Some(access_token) = token_response.access_token {
            println!("Access token received!");
            return Ok(access_token);
        } else if let Some(error) = token_response.error {
            if error == "authorization_pending" {
                sleep(Duration::from_secs(interval)).await;
            } else {
                return Err(format!(
                    "Error: {} - {}",
                    error,
                    token_response.error_description.unwrap_or_default()
                )
                .into());
            }
        }
    }
}

pub async fn authenticate_user() {
    let client = Client::new();

    let device_code_response = get_device_code(&client).await.unwrap();

    let access_token = poll_for_access_token(
        &client,
        device_code_response.device_code,
        device_code_response.interval,
    )
    .await
    .unwrap();

    let entry = Entry::new("codepath", "auth").unwrap();
    entry.set_password(access_token.as_str()).unwrap();

    println!("Access Token: {}", access_token);
}
