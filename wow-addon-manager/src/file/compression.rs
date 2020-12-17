use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use std::io::prelude::*;
use zip::ZipArchive;

pub fn unpack_tar(in_path: &str, out_path: &str) -> Result<(), io::Error> {
    let file = File::open(in_path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(out_path)?;
    Ok(())
}

pub fn unpack_zip(in_path: &str, out_path: &str) -> Result<(), io::Error> {
    let zipfile = File::open(&in_path)?;
    let mut archive = zip::ZipArchive::new(zipfile)?;
    for i in 0 .. archive.len() {
        let mut file = archive.by_index(i)?;
        let file_out_path = match file.enclosed_name() {
            Some(path) => Path::new(out_path).join(path).to_owned(),
            None       => continue,
        };
        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&file_out_path)?;
        } else {
            if let Some(parent) = file_out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent)?;
                }
            }
            let mut out_file = fs::File::create(&file_out_path)?;
            io::copy(&mut file, &mut out_file)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(
                    &file_out_path,
                    fs::Permissions::from_mode(mode),
                )?;
            }
        }
    }
    Ok(())
}
