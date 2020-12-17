use reqwest::Error;
use reqwest::Client;

mod net;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let releases = net::github::get_releases(
        &client,
        "Kumodatsu",
        "wow-addon-manager"
    ).await?;
    println!("{:?}", releases);
    
    Ok(())
}
