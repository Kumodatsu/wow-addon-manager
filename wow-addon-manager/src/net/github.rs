use serde::Deserialize;
use reqwest::Client;
use reqwest::Error;
use reqwest::header;
use chrono::DateTime;

#[derive(Debug, Deserialize)]
pub struct Release {
    pub url:          String,
    pub id:           u32,
    pub name:         String,
    pub prerelease:   bool,
    pub published_at: String,
    pub zipball_url:  String,
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

pub fn get_latest_release(
    releases:          &Vec<Release>,
    allow_prereleases: bool
) -> Option<&Release> {
    releases.into_iter()
        .filter(|r| allow_prereleases || !r.prerelease)
        .max_by_key(|r| DateTime::parse_from_rfc3339(&r.published_at)
            .expect("Encountered invalid date format in GitHub release."))
}
