use anyhow::{Context, Result};
use keyring::Entry;
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;
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

/// Opens the given URL in the default web browser.
fn open_url(url: &str) -> Result<()> {
    match std::env::consts::OS {
        "windows" => Command::new("cmd").args(["/C", "start", url]).spawn(),
        "macos" => Command::new("open").arg(url).spawn(),
        "linux" => Command::new("xdg-open").arg(url).spawn(),
        _ => return Ok(()),
    }
    .context("Failed to open browser")?;
    Ok(())
}

/// Retrieves the device code needed for OAuth authentication.
async fn get_device_code(client: &Client) -> Result<DeviceCodeResponse> {
    let response = client
        .post(DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .form(&[("client_id", CLIENT_ID), ("scope", SCOPE)])
        .send()
        .await
        .context("Failed to request device code")?;

    let device_code_response = response
        .json::<DeviceCodeResponse>()
        .await
        .context("Failed to parse device code response")?;

    println!("Please enter the code: {}", device_code_response.user_code);
    println!("Opening browser for authorization...");

    open_url(&device_code_response.verification_uri)?;

    Ok(device_code_response)
}

/// Creates a new repository from a template in the specified organization.
pub fn create_repository_from_template(
    org: &str,
    template_repo: &str,
    new_repo_name: &str,
    user_token: &str,
) -> Result<()> {
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
    headers.insert(
        USER_AGENT,
        "MyApp/1.0"
            .parse()
            .context("Failed to parse User-Agent header")?,
    );

    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", user_token))
        .header("Accept", "application/vnd.github.v3+json")
        .headers(headers)
        .json(&request_body)
        .send()
        .context("Failed to send repository creation request")?;

    if response.status().is_success() {
        println!(
            "Repository '{}' created successfully from template '{}'.",
            new_repo_name, template_repo
        );
    } else {
        let error_message = response.text().context("Failed to read error response")?;
        anyhow::bail!("Failed to create repository: {}", error_message);
    }

    Ok(())
}

/// Polls the GitHub API for an access token based on the device code.
async fn poll_for_access_token(
    client: &Client,
    device_code: String,
    interval: u64,
) -> Result<String> {
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
            .await
            .context("Failed to poll for access token")?;

        let token_response = response
            .json::<AccessTokenResponse>()
            .await
            .context("Failed to parse access token response")?;

        if let Some(access_token) = token_response.access_token {
            println!("Access token received!");
            return Ok(access_token);
        } else if let Some(error) = token_response.error {
            if error == "authorization_pending" {
                sleep(Duration::from_secs(interval)).await;
            } else {
                anyhow::bail!(
                    "Error: {} - {}",
                    error,
                    token_response.error_description.unwrap_or_default()
                );
            }
        }
    }
}

/// Authenticates the user and stores the access token securely.
pub async fn authenticate_user() -> Result<()> {
    let client = Client::new();

    let device_code_response = get_device_code(&client)
        .await
        .context("Failed to get device code")?;

    let access_token = poll_for_access_token(
        &client,
        device_code_response.device_code,
        device_code_response.interval,
    )
    .await
    .context("Failed to poll for access token")?;

    let entry = Entry::new("codepath", "auth").context("Failed to create keyring entry")?;
    entry
        .set_password(&access_token)
        .context("Failed to set access token in keyring")?;

    println!("Access Token: {}", access_token);
    Ok(())
}
