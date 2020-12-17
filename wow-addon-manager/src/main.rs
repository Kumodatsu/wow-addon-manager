use reqwest::Client;

mod net;
mod file;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let data = net::curseforge::get_addon_data(&client, 61284)
        .await?;
    println!("{:?}", data);
    Ok(())
}
