use std::{fs, io::{Read, Write}, path::{Path, PathBuf}};

struct Config {
    envvars: toml::value::Array,
    directories: toml::value::Array
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
    println!("{}\n{}", config_dir_path.display(), config_file_path.display());
    let already_setup = match config_dir_path.try_exists() {
        Ok(b) => b,
        Err(e) => {
            println!("{}", e);
            false
        }
    };
    if !already_setup {
        println!("blabla");
        fs::create_dir_all(&config_dir_path).unwrap();
        let mut file = fs::File::create(&config_file_path).unwrap();
        file.write_all(config_dir_path.to_str().unwrap().as_bytes());
    }

    let mut file = fs::File::open(config_file_path).unwrap();
    let mut src = String::new();
    file.read_to_string(&mut src);
    println!("{}", src);
    let toml_config = toml::from_str(&src.as_str());
}