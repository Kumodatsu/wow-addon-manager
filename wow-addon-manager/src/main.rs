use fs_extra::dir::CopyOptions;
use reqwest::Client;
use flexi_logger::{Duplicate, Logger};

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

    Logger::with_str("info")
        .check_parser_error()
        .unwrap()
        .log_to_file()
        .directory("log")
        .suffix("log")
        .format_for_files(|write, now, record| {
            write!(
                write,
                "[{}] {} from {}:{}: {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                &record.args()
            )
        })
        .duplicate_to_stderr(
            if args.is_present("verbose") {
                Duplicate::All
            } else {
                Duplicate::Warn
            }
        )
        .format(|write, now, record| {
            write!(
                write,
                "[{}] {}: {}",
                now.now().format("%H:%M:%S"),
                record.level(),
                &record.args()
            )
        })
        .start()
        .unwrap_or_else(|e| panic!("Failed to initalize logger: {}", e));    

    let config_path = args.value_of("config").unwrap_or("config.yaml");
    log::info!("Using configuration file: {}", config_path);

    let config = match file::config::read_config(config_path) {
        Ok(config) => config,
        Err(err)   => {
            log::error!("Failed to read config file: {}", err);
            return Ok(());
        },
    };

    std::fs::create_dir_all("temp")?;

    let client = Client::new();

    if let Some(curseforge) = config.curseforge {
        log::info!("Starting with CurseForge addons.");
        for project_id in curseforge {
            log::info!("Starting with CurseForge addon with project ID {}.",
                project_id);
            let data = match net::curseforge::get_addon_data(
                &client,
                project_id
            ).await {
                Ok(data) => data,
                Err(err) => {
                    log::warn!(
                        "Skipping CurseForge project {} as addon info could not be downloaded: {}",
                        project_id,
                        err,
                    );
                    continue;
                }
            };
            let latest = net::curseforge::get_latest_release(&data.latest_files,
                false);
            if let Some(latest) = latest {
                let download_path = "temp/release.zip";
                let unpack_path   = format!("temp/curse_{}", project_id);
                match net::download::download(
                    &client,
                    &latest.download_url,
                    download_path
                ).await {
                    Ok(())   => { log::info!("Downloaded {}.", &data.name); },
                    Err(err) => {
                        log::warn!(
                            "Skipping {} as it could not be downloaded: {}",
                            &data.name,
                            err,
                        );
                        continue;
                    }
                }
                match file::compression::unpack_zip(
                    download_path,
                    &unpack_path,
                ) {
                    Ok(())   => { log::info!("{} unzipped.", &data.name); },
                    Err(err) => {
                        log::warn!(
                            "Skipping {} because it failed to unzip: {}",
                            &data.name,
                            err,
                        );
                        continue;
                    }
                };
                std::fs::remove_file(download_path)?;
            } else {
                log::warn!(
                    "Skipping {} as its latest version could not be found.",
                    &data.name,
                );
            }
        }
        log::info!("Finished with CurseForge addons.");
    }

    if let Some(github) = config.github {
        log::info!("Starting with GitHub addons.");
        for url in github {
            let (owner, repo) = net::github::get_owner_and_repo(&url);
            log::info!("Starting with {}.", repo);
            let releases = match net::github::get_releases(
                &client,
                owner,
                repo
            ).await {
                Ok(releases) => releases,
                Err(err)     => {
                    log::warn!(
                        "Skipping {} as its releases could not be found: {}",
                        repo,
                        err,
                    );
                    continue;
                }
            };
            let latest = net::github::get_latest_release(
                &releases,
                true
            );
            if let Some(latest) = latest {
                let download_path = "temp/release.tar.gz";
                let unpack_path   = format!("temp/github_{}", repo);
                match net::download::download(
                    &client,
                    &latest.tarball_url,
                    download_path
                ).await {
                    Ok(())   => log::info!("Downloaded {}.", repo),
                    Err(err) => {
                        log::warn!(
                            "Skipping {} as it could not be downloaded: {}",
                            repo,
                            err,
                        );
                        continue;
                    }
                }
                match file::compression::unpack_tar(
                    download_path,
                    &unpack_path
                ) {
                    Ok(())   => log::info!("Unpacked {}.", repo),
                    Err(err) => {
                        log::warn!(
                            "Skipping {} as it could not be unpacked: {}",
                            repo,
                            err,
                        );
                    }
                }
                // TODO: Make sure this happens in all cases
                std::fs::remove_file(download_path)?;
            } else {
                log::warn!(
                    "Skipping {} as its latest release could not be found.",
                    repo,
                );
            }
        }
    }
    
    let temp_path = "temp";
    let addons    = match file::detection::detect_addons(&temp_path) {
        Ok(addons) => addons,
        Err(err)   => {
            log::error!(
                "Could not access addon files: {}",
                err,
            );
            return Ok(());
        },
    };

    let copy_options = CopyOptions::new();
    let addons_path  = std::path::PathBuf::from(&config.path);

    for addon in addons {
        log::info!("Copying {}.", addon.name);

        let p = addon.path.parent().unwrap().join(&addon.name);
        if let Err(err) = std::fs::rename(&addon.path, &p) {
            log::error!(
                "Failed to rename path {}: {}",
                &addon.path.to_str().unwrap(),
                err
            );
            return Ok(());
        }
        let target_path = addons_path.join(&addon.name);
        if target_path.exists() {
            std::fs::remove_dir_all(target_path)?;
        }
        if let Err(err) = fs_extra::dir::copy(&p, &addons_path, &copy_options) {
            log::error!(
                "Could not copy {}: {}",
                addon.name,
                err,
            );
            continue;
        }
    }

    std::fs::remove_dir_all(temp_path)?;

    println!("Finished updating addons.");

    Ok(())
}
