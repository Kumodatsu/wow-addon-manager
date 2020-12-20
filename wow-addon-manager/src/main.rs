use fs_extra::dir::CopyOptions;
use reqwest::Client;

#[macro_use]
extern crate clap;
use clap::App;

mod error;
mod net;
mod file;

use error::error::AddonError;

#[tokio::main]
async fn main() -> Result<(), AddonError> {
    let yaml = load_yaml!("cmdargs.yaml");
    let args = App::from_yaml(yaml).get_matches();

    let config_path = args.value_of("config").unwrap_or("config.yaml");

    let config = file::config::read_config(config_path)
        .expect("Could not read config file.");
    std::fs::create_dir_all("temp")?;

    let client = Client::new();

    if let Some(curseforge) = config.curseforge {
        for project_id in curseforge {
            println!("Starting with CurseForge addon {}.", project_id);
            let data = net::curseforge::get_addon_data(&client, project_id)
                .await?;
            let latest = net::curseforge::get_latest_release(&data.latest_files,
                false);
            if let Some(latest) = latest {
                let download_path = "temp/release.zip";
                let unpack_path   = format!("temp/curse_{}", project_id);
                net::download::download(&client, &latest.download_url,
                        download_path)
                    .await
                    .expect("Failed to download.");
                file::compression::unpack_zip(
                    download_path,
                    &unpack_path,
                )?;
                std::fs::remove_file(download_path)?;
            }
            println!("Done.");
        }
    }

    if let Some(github) = config.github {
        for url in github {
            let (owner, repo) = net::github::get_owner_and_repo(&url);
            println!("Starting with repo {}", repo);
            let releases      = net::github::get_releases(
                &client,
                owner,
                repo
            ).await?;
            let latest = net::github::get_latest_release(
                &releases,
                true
            );
            if let Some(latest) = latest {
                let download_path = "temp/release.tar.gz";
                let unpack_path   = format!("temp/github_{}", repo);
                net::download::download(&client, &latest.tarball_url,
                        download_path)
                    .await
                    .expect("Failed to download.");
                file::compression::unpack_tar(
                    download_path,
                    &unpack_path
                )?;
                std::fs::remove_file(download_path)?;
            }
            println!("Done.");
        }
    }
    
    let temp_path = "temp";
    let addons    = file::detection::detect_addons(&temp_path)?;

    let copy_options = CopyOptions::new();
    let addons_path  = std::path::PathBuf::from(&config.path);

    for addon in addons {
        println!("Copying {}.", addon.name);

        let p = addon.path.parent().unwrap().join(&addon.name);
        std::fs::rename(&addon.path, &p)
            .expect("Could not rename folder.");
        let target_path = addons_path.join(&addon.name);
        if target_path.exists() {
            std::fs::remove_dir_all(target_path)?;
        }
        fs_extra::dir::copy(&p, &addons_path, &copy_options)
            .expect("Could not copy files.");
    }

    std::fs::remove_dir_all(temp_path)?;

    println!("Finished updating addons.");

    Ok(())
}
