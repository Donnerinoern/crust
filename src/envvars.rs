use std::{env, fs::File, io::{BufWriter, Write}, path::PathBuf};

use chrono::format::{DelayedFormat, StrftimeItems};

pub fn handle_envvars(toml_envvars: toml::value::Array, datetime: &DelayedFormat<StrftimeItems>) {
    let envvars_vec: Vec<&str> = toml_envvars.iter().map(|x| x.as_str().unwrap()).collect();
    let envvars = env::vars();
    let chosen_envvars = envvars.filter(|x| envvars_vec.contains(&x.0.as_str()));
    let mut file_path = PathBuf::from("/tmp/crust");
    file_path.push(datetime.to_string());
    file_path.push("envvars.crust");
    let envvar_file = File::create_new(file_path).unwrap();
    let mut writer = BufWriter::new(envvar_file);
    for var in chosen_envvars {
        // println!("{}", var.0);
        writer.write_all(var.0.as_bytes()).unwrap();
        writer.write_all(b"=").unwrap();
        writer.write_all(var.1.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}
