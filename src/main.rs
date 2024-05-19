#![feature(string_remove_matches)]
#![feature(fs_try_exists)]

mod filesystem;
mod envvars;
mod error;
mod pack;
mod unpack;

use std::{env, fs, io::{Read, Write}, path::{Path, PathBuf}};
use pack::archive;
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

enum Mode {
    None,
    Pack,
    Unpack
}

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let args: Vec<&str> = all_args[1..all_args.len()-1].iter().map(|a| a.as_str()).collect();
    let tmp_path: &Path = &get_tmppath();
    let mut mode = Mode::None;
    let file_path = match args.len() {
        1 | 2 => {
            for arg in &args[..] {
                mode = handle_arg(arg);
            }
            Ok(PathBuf::from(all_args.last().unwrap()))
        }
        _ => {
            mode = Mode::None;
            Err("No valid file path")
        }
    };
    match mode {
        Mode::Pack => {
            pack(tmp_path, &file_path.unwrap());
        }
        Mode::Unpack => {
            unpack(tmp_path, &file_path.unwrap());
        }
        Mode::None => {
            help();
        }
    }
}

fn pack(tmp_path: &Path, output_path: &Path) {
    let config_dir_path = home::home_dir().unwrap_or_default().join(".config/crust");
    let config_file_path = config_dir_path.join("config.toml");
    match config_file_path.try_exists() {
        Ok(exists) => {
            if !exists {
                if let Err(e) = fs::create_dir_all(&config_dir_path) {
                    println!("{}", e);
                }
                let file = fs::File::create(&config_file_path);
                match file {
                    Ok(mut f) => {
                        f.write_all(get_example_config().as_bytes()).unwrap();
                    }
                    Err(e) => println!("{}", e)
                }
            }
        }
        Err(e) => println!("{}", e)
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

    handle_paths(config.backup.paths, tmp_path);
    handle_envvars(config.backup.envvars, tmp_path);
    archive(tmp_path);
}

fn unpack(tmp_path: &Path, input_path: &Path) {
    unpack::decompress(tmp_path, input_path)
}

fn handle_arg(arg: &str) -> Mode {
    match arg {
        "-p" | "--pack" => {
            Mode::Pack
        }
        "-u" | "--unpack" => {
            Mode::Unpack
        }
        _ => {
            println!("Unknown arg: {}", arg);
            Mode::None
        }
    }
}

fn get_tmppath() -> PathBuf {
    let mut tmp_pathbuf = PathBuf::new();
    tmp_pathbuf.push("/tmp/crust");
    tmp_pathbuf.push(get_datetime());
    tmp_pathbuf
}

pub fn get_datetime() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

fn help() {
    println!("Usage: crust [<args...>] <archive>

Args:
-h                  --help
-p                  --pack
-u                  --unpack");
}

fn get_example_config() -> String {
    "# Example config

[backup]
envvars = [
    # \"EDITOR\",
    # \"TERM\"
]
paths = [
    # \"~/.config/nvim\",
    # \"/etc/makepkg.conf\"
]".to_string()
}
