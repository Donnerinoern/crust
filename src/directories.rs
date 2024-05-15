use std::{path::{Path, PathBuf}, str::FromStr};
use chrono::offset::Local;
use home::home_dir;
use crate::error::Errors;

pub fn handle_directories(directories: toml::value::Array) {
    let tmp_path: &Path = Path::new("/tmp/crust/");
    let datetime = Local::now();
    let formatted_datetime = datetime.format("%Y%m%d_%H%M");
    let mut dir_vec: Vec<PathBuf> = Vec::new();

    for directory in directories.iter() {
        dir_vec.push(format_dir_path(directory))
    }
    for t in dir_vec {
        println!(": {}", t.display());
    }
}

fn format_dir_path(dir_string: &toml::Value) -> PathBuf {
    let erros = Errors::new();
    let dir_str = dir_string.as_str().unwrap_or_default();
    let mut dir_mut_string = String::from_str(dir_str).unwrap_or_default();
    dir_mut_string.replace("~", home_dir().unwrap_or_default().to_str().unwrap_or_default());

    match PathBuf::from_str(dir_str) {
        Ok(p) => ,
        Err(e) => {
            println!("{}", e);
            PathBuf::new()
        }
    }
}
