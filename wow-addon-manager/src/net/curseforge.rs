use serde::Deserialize;
use reqwest::Client;
use reqwest::header;
use chrono::DateTime;

#[derive(Debug, Deserialize)]
pub struct Release {
    pub id:                  u32,
    #[serde(rename(deserialize = "displayName"))]
    pub display_name:        String,
    #[serde(rename(deserialize = "fileName"))]
    pub file_name:           String,
    #[serde(rename(deserialize = "fileDate"))]
    pub file_date:           String,
    #[serde(rename(deserialize = "releaseType"))]
    pub release_type:        u32,
    #[serde(rename(deserialize = "downloadUrl"))]
    pub download_url:        String,
    #[serde(rename(deserialize = "gameVersion"))]
    pub game_version:        Vec<String>,
    #[serde(rename(deserialize = "gameVersionFlavor"))]
    pub game_version_flavor: String,
}

#[derive(Debug, Deserialize)]
pub struct AddonData {
    pub id:           u32,
    pub name:         String,
    #[serde(rename(deserialize = "gameId"))]
    pub game_id:      u32,
    pub summary:      String,
    #[serde(rename(deserialize = "latestFiles"))]
    pub latest_files: Vec<Release>,
}

fn get_project_url(project_id: u32) -> String {
    format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}", project_id)
}

pub async fn get_addon_data(
    client: &Client,
    project_id: u32
) -> Result<AddonData, reqwest::Error> {
    let url      = get_project_url(project_id);
    let response = client
        .get(&url)
        .header(header::USER_AGENT, env!("CARGO_PKG_NAME"))
        .header(header::CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=UTF-8")
        .send()
        .await?;
    let data: AddonData = response.json().await?;
    Ok(data)
}

pub fn get_latest_release(
    releases:          &Vec<Release>,
    allow_prereleases: bool,
) -> Option<&Release> {
    releases.into_iter()
        .filter(|r| (allow_prereleases || r.release_type == 1)
            && r.game_version_flavor == "wow_retail")
        .max_by_key(|r| DateTime::parse_from_rfc3339(&r.file_date)
            .expect("Encountered invalid date format in CurseForge release."))
}

