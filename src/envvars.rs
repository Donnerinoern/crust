use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

pub fn handle_envvars(toml_envvars: &toml::value::Array, tmp_pathbuf: &Path) {
    let envvars_vec: Vec<&str> = toml_envvars.iter().map(|x| x.as_str().unwrap()).collect();
    let envvars = env::vars();
    let chosen_envvars = envvars.filter(|x| envvars_vec.contains(&x.0.as_str()));
    let mut file_path = tmp_pathbuf.to_path_buf();
    file_path.push("envvars.crust");
    let envvar_file = File::create_new(file_path).unwrap();
    let mut writer = BufWriter::new(envvar_file);
    for var in chosen_envvars {
        writer.write_all(var.0.as_bytes()).unwrap();
        writer.write_all(b"=").unwrap();
        writer.write_all(var.1.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}
