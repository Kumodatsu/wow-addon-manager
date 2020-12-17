use reqwest::Client;

mod net;
mod file;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let releases = net::github::get_releases(
        &client, 
        "Kumodatsu",
        "CharacterSheet",
    ).await?;
    let latest_release = net::github::get_latest_release(&releases, true);
    if let Some(latest_release) = latest_release {
        println!("{:?}", latest_release);
        net::download::download(&client, &latest_release.tarball_url, "release.tar.gz")
            .await
            .expect("Could not download file.");
        file::compression::unpack("release.tar.gz", "release")
            .expect("Could not unpack release archive.");
    }
    Ok(())
}
