use serde::Deserialize;
use reqwest::Client;
use reqwest::header;

/// Represents an addon from the Tukui service.
/// 
/// The objects returned by actual Tukui API calls have more fields, but their
/// types and names are inconsistent. If these fields are ever needed, this
/// struct will have to be deserialized manually.
#[derive(Debug, Deserialize)]
pub struct Addon {
    pub name:       String,
    pub small_desc: String,
    pub author:     String,
    pub version:    String,
    pub url:        String,
}

pub async fn get_addon(
    client:     &Client,
    addon_name: &str,
) -> Result<Option<Addon>, reqwest::Error> {
    // ElvUI and Tukui need special treatment because yes
    let (url, vip) = if addon_name == "ElvUI" {
        ("https://www.tukui.org/api.php?ui=elvui", true)
    } else if addon_name == "Tukui" {
        ("https://www.tukui.org/api.php?ui=tukui", true)
    } else {
        ("https://www.tukui.org/api.php?addons", false)
    };
    let response = client
        .get(url)
        .header(header::USER_AGENT, env!("CARGO_PKG_NAME"))
        .send()
        .await?;
    let addons: Vec<Addon> = if vip {
        let addon = response.json().await?;
        vec![addon]
    } else {
        response.json().await?
    };
    for addon in addons {
        if addon.name == addon_name {
            return Ok(Some(addon));
        }
    } 
    Ok(None)
}
