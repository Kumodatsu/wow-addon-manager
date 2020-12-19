use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AddonLocation {
    pub path:  PathBuf,
    pub name:  String,
}

pub fn detect_addons<P: AsRef<Path>>(
    path: &P
) -> Result<Vec<AddonLocation>, io::Error> {
    let mut addons: Vec<AddonLocation> = Vec::new();
    let mut search_dirs: Vec<PathBuf>  = Vec::new();
    search_dirs.push(path.as_ref().to_path_buf());
    while search_dirs.len() != 0 {
        let dir = search_dirs.pop().unwrap();
        let mut n_dirs = 0;
        for file in dir.read_dir()? {
            let file      = file?;
            let file_type = file.file_type()?;
            let file_path = file.path();
            if file_type.is_dir() {
                search_dirs.push(file_path);
                n_dirs += 1;
            } else if file_type.is_file() {
                if let Some(ext) = file_path.extension() {
                    if ext == "toc" {
                        addons.push(AddonLocation {
                            path: file_path.parent().unwrap().to_path_buf(),
                            name: String::from(file_path.file_stem()
                                .and_then(OsStr::to_str).unwrap()),
                        });
                        for _ in 0 .. n_dirs {
                            search_dirs.pop();
                        }
                        break;
                    }
                }
            }
        }
    }
    Ok(addons)
}
