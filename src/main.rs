mod filesystem;
mod envvars;
mod error;
mod compress;

use std::{fs, io::{Read, Write}, path::PathBuf};
use compress::archive;
use serde::Deserialize;
use chrono::Local;
use crate::{filesystem::handle_paths, envvars::handle_envvars};

#[derive(Deserialize)]
struct Backup {
    envvars: toml::value::Array,
    paths: toml::value::Array
}

#[derive(Deserialize)]
struct Config {
    backup: Backup
}

fn main() {
    let config_dir_path = match home::home_dir() {
        Some(p) => p.join(".config/crust"),
        None => {
            println!("Unable to find your home directory!");
            PathBuf::new()
        }
    };
    let config_file_path = config_dir_path.join("config.toml");
    let already_setup = match config_dir_path.try_exists() {
        Ok(b) => b,
        Err(e) => {
            println!("{}", e);
            false
        }
    };
    if !already_setup {
        fs::create_dir_all(&config_dir_path).unwrap();
        let mut file = fs::File::create(&config_file_path).unwrap();
        file.write_all(config_dir_path.to_str().unwrap().as_bytes()); // Write a valid TOML
                                                                           // config instead
    }

    let mut file = fs::File::open(config_file_path).unwrap();
    let mut src = String::new();
    file.read_to_string(&mut src);
    let config: Config  = match toml::from_str(src.as_str()) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let datetime = Local::now().format("%Y%m%d_%H%M");
    let mut tmp_pathbuf = PathBuf::new();
    tmp_pathbuf.push("/tmp/crust");
    tmp_pathbuf.push(datetime.to_string());
    handle_paths(config.backup.paths, &tmp_pathbuf);
    handle_envvars(config.backup.envvars, &tmp_pathbuf);
    archive(&tmp_pathbuf);
}
