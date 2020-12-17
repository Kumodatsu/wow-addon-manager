use reqwest::Client;
use reqwest::header;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum DownloadError {
    RequestError(reqwest::Error),
    StatusError(reqwest::StatusCode),
    IOError(io::Error),
}

impl From<reqwest::Error> for DownloadError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}

impl From<reqwest::StatusCode> for DownloadError {
    fn from(status: reqwest::StatusCode) -> Self {
        Self::StatusError(status)
    }
}

impl From<io::Error> for DownloadError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

pub async fn download(
    client: &Client,
    url:    &str,
    path:   &str,
) -> Result<(), DownloadError> {
    let response = client
        .get(url)
        .header(header::USER_AGENT, env!("CARGO_PKG_NAME"))
        .send()
        .await?;
    let status = response.status();
    if status == 200 {
        let content = response.bytes().await?;
        let mut file = File::create(path)?;
        match file.write_all(&content) {
            Ok(())     => Ok(()),
            Err(error) => Err(DownloadError::from(error)),
        }
    } else {
        Err(DownloadError::from(status))
    }
}
