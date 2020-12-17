use std::fs::File;
use std::io;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn unpack(in_path: &str, out_path: &str) -> Result<(), io::Error> {
    let file = File::open(in_path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(out_path)?;
    Ok(())
}
