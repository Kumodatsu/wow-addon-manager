use serde::Deserialize;
use reqwest::Error;
use reqwest::Client;
use reqwest::header;

#[derive(Deserialize, Debug)]
struct Release {
    url:        String,
    id:         u32,
    name:       String,
    prerelease: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases",
        owner = "Kumodatsu",
        repo  = "wow-addon-manager"
    );
    println!("URL: {}", url);
    let response = client
        .get(&url)
        .header(header::USER_AGENT, "wow-addon-manager")
        .send()
        .await?;
    let status = response.status();
    println!("{:?}", status);
    let users: Vec<Release> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
