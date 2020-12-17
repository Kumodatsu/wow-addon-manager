use serde::Deserialize;
use reqwest::Client;
use reqwest::Error;
use reqwest::header;

#[derive(Deserialize, Debug)]
pub struct Release {
    url:        String,
    id:         u32,
    name:       String,
    prerelease: bool,
}

pub async fn get_releases(
    client: &Client,
    owner:  &str,
    repo:   &str
) -> Result<Vec<Release>, Error> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases",
        owner = owner,
        repo  = repo,
    );
    let response = client
        .get(&url)
        .header(header::USER_AGENT, env!("CARGO_PKG_NAME"))
        .send()
        .await?;
    let releases: Vec<Release> = response.json().await?;
    Ok(releases)
}
