#![feature(string_remove_matches)]
#![feature(fs_try_exists)]

mod filesystem;
mod envvars;
mod error;
mod pack;
mod unpack;
mod network;

use std::{env::{self}, fs, io::{Read, Write}, net::Ipv4Addr, path::PathBuf, str::FromStr};
use serde::Deserialize;
use chrono::Local;
use toml::de::Error;

#[derive(Deserialize)]
struct Network {
    ip_addr: Option<Ipv4Addr>,
    port: Option<u32>
}

#[derive(Deserialize)]
struct Backup {
    envvars: toml::value::Array,
    paths: toml::value::Array
}

#[derive(Deserialize)]
struct TomlConfig {
    network: Option<Network>,
    backup: Backup
}

struct Config {
    mode: Mode,
    network: Option<Network>,
    file_path: Option<PathBuf>,
    tmp_path: PathBuf
}

enum Mode {
    None,
    Pack,
    Unpack,
    Listen,
    Connect
}

fn main() {
    let config = get_config_and_handle_args();
    match config.0.mode {
        Mode::Pack => {
            pack::pack(&config.0, &config.1);
        }
        Mode::Unpack => {
            // unpack(&config.0);
            unpack::decompress(&config.0);
        }
        Mode::None => {
            help();
        }
        Mode::Listen => {
            network::create_tcp_listener();
        }
        Mode::Connect => {
            network::create_tcp_stream();
        }
    }
}

fn parse_toml_config() -> Result<TomlConfig, Error> {
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
    toml::from_str(src.as_str())
}

fn parse_mode(mode: &str) -> Mode {
    match mode {
        "pack" => {
            Mode::Pack
        }
        "unpack" => {
            Mode::Unpack
        }
        "listen" => {
            Mode::Listen
        }
        "connect" => {
            Mode::Connect
        }
        _ => {
            println!("Error: Unknown mode '{}'", mode);
            Mode::None
        }
    }
}

fn get_config_and_handle_args() -> (Config, TomlConfig) {
    let mut config = Config {
        mode: Mode::None,
        network: None,
        file_path: None,
        tmp_path: get_tmppath()
    };
    let toml_config = parse_toml_config().unwrap();
    let mut args = env::args().peekable();
    match env::args().len() {
        1 => config.mode = Mode::None,
        2 => {
            let mut mode = parse_mode(&args.nth(1).unwrap());
            if let Mode::Unpack = mode {
                println!("Error: Missing file path");
                mode = Mode::None;
            }
            config.mode = mode;
        }
        _ => {
            config.mode = parse_mode(&args.nth(1).unwrap());
            let mut ip_addr: Option<Ipv4Addr> = None;
            let mut port: Option<u32> = None;
            while let Some(a) = args.next() {
                match a.as_str() {
                    "-a" | "--address" => {
                        ip_addr = Some(Ipv4Addr::from_str(&args.next().unwrap()).unwrap());
                    }
                    "-p" | "--port" => {
                        port = Some(args.next().unwrap().parse().unwrap());
                    }
                    _ => {
                        if args.peek().is_none() {
                            config.file_path = Some(a.into());
                        } else {
                            println!("Error: Unknown argument '{}'", a);
                            config.mode = Mode::None;
                        }
                    }
                }
            }
            match config.mode {
                Mode::Listen | Mode::Connect => {
                    let network: Network = match toml_config.network {
                        Some(ref toml_network) => {
                            let mut n = Network {
                                ip_addr: toml_network.ip_addr,
                                port: toml_network.port
                            };
                            if ip_addr.is_some() {
                                n.ip_addr = ip_addr;
                            }
                            if port.is_some() {
                                n.port = port;
                            }
                            n
                        }
                        None => {
                            Network {
                                ip_addr,
                                port
                            }
                        }
                    };
                    if network.ip_addr.is_none() {
                        config.mode = Mode::None;
                        println!("Error: Missing IP-address");
                    }
                    if network.port.is_none() {
                        config.mode = Mode::None;
                        println!("Error: Missing port");
                    }
                    config.network = Some(network);
                }
                _ => {

                }
            }
        }
    }
    (config, toml_config)
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
    println!("Usage:
  crust command [options...] [<arg>] [<file>]

Commands:
  pack                      -- packs the files/directories declared in config.toml and creates a compressed archive
  unpack                    -- unbacks the compressed archive and moves the contained files to their respective paths
  listen                    -- listens for incoming connections on specified address and port
  connect                   -- connect to the specified address and port

Options:
  --help    | -h            -- prints this message
  --address | -a <addr>     -- specifies IP-address for incoming/outgoing connection(s)
  --port    | -p <port>     -- specifies port for incoming/outgoing connection(s)

Example usage:
  crust pack
  crust unpack archive.tar.zst
  crust listen -a localhost -p 5000
  crust connect");
}

fn get_example_config() -> String {
    "# Example config
[network]
ip_addr = \"localhost\"
port = 5000

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
