use tar::Builder;
use zstd::stream::copy_encode;
use std::{collections::VecDeque, env, fs::File, path::Path};
use crate::{get_datetime, TomlConfig, Config, filesystem::handle_paths, envvars::handle_envvars};

pub fn pack(config: &Config, toml_config: &TomlConfig) {
    handle_paths(&toml_config.backup.paths, &config.tmp_path);
    handle_envvars(&toml_config.backup.envvars, &config.tmp_path);
    archive(&config.tmp_path);
}

fn archive(tmp_path: &Path) {
    let byte_vec: VecDeque<u8> = VecDeque::new();
    let mut builder = Builder::new(byte_vec);
    builder.append_dir_all(".", tmp_path).unwrap();
    builder.finish().unwrap();
    compress(builder.get_mut());
}

fn compress(byte_vec: &mut VecDeque<u8>) {
    let mut path = env::current_dir().unwrap();
    path.push(get_datetime());
    path.set_extension("tar.zst");
    let file = File::create_new(path).unwrap();
    copy_encode(byte_vec, file, 0).unwrap();
}
