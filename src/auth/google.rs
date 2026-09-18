use serde::Deserialize;
use serde_json::from_str;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::auth;

const REDIRECT_URI: &str = "http://127.0.0.1:8080/callback";
const SCOPE: &str = "https://www.googleapis.com/auth/drive.file";

#[derive(Debug, Deserialize)]
struct CredentialsFile {
    installed: InstalledCredentials,
}

#[derive(Debug, Deserialize)]
struct InstalledCredentials {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    client_secret : String
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: String,
    token_type: String,
}

pub async fn gdrive_auth() -> Result<(), ()> {
    let contents = std::fs::read_to_string("src/credentials.json").expect("couldn't read the file");

    let credentials: CredentialsFile = from_str(&contents).expect("Invalid Credentials");

    println!("Starting gdrive authentication");

    let auth_url = build_auth_url(&credentials);

    println!("opening browser...");

    webbrowser::open(&auth_url).expect("failed to open the browser");

    println!(
        "Complete the authentication, if browser doesn't open paste this link and open in your browser : {:?}",
        auth_url
    );

    let code = wait_for_callback()
        .await
        .expect("failed to get the callback");

    println!("Authorization code received!");

    let tokens = exchange_code(&code, &credentials).await.expect("token exchange failed!");
    
    println!("Google authentication successful!");
    Ok(())
}

fn build_auth_url(credentials: &CredentialsFile) -> String {
    format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        credentials.installed.auth_uri,
        urlencoding::encode(&credentials.installed.client_id),
        urlencoding::encode(REDIRECT_URI),
        urlencoding::encode(SCOPE),
    )
}

async fn wait_for_callback() -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Waiting for Google authorization...");

    let (mut socket, _) = listener.accept().await?;

    let mut buffer = [0u8; 4096];

    let bytes_read = socket.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received request:");

    let request_line = request
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid HTTP request"))?;

    let path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Invalid HTTP request Line"))?;

    let url = url::Url::parse(&format!("http://localhost{}", path))?;

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .ok_or_else(|| anyhow::anyhow!("Authorization code not found"))?;

    let response = "HTTP/1.1 200 OK\r\n\
    Content-Type: text/plain\r\n\
    Connection: close\r\n\
    \r\n\
    Transit authorization successful. You can close this window.";

    socket.write_all(response.as_bytes()).await?;

    Ok(code)
}

async fn exchange_code(
    code: &str,
    credentials: &CredentialsFile,
) -> anyhow::Result<TokenResponse> {
    let client = reqwest::Client::new();

    let params = [
        ("code", code),
        ("client_id", credentials.installed.client_id.as_str()),
        ("client_secret", credentials.installed.client_secret.as_str()),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post(&credentials.installed.token_uri)
        .json(&params)
        .send()
        .await?;

    let tokens = response
        .error_for_status()?
        .json::<TokenResponse>()
        .await?;

    Ok(tokens)
}