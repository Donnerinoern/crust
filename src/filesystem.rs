use std::{fs::{copy, read_dir, DirBuilder}, path::PathBuf, str::FromStr};
use chrono::{format::{DelayedFormat, StrftimeItems}, offset::Local};
use home::home_dir;

pub fn handle_paths(fs_entries: toml::value::Array, datetime: &DelayedFormat<StrftimeItems>) {
    // let tmp_path: &Path = Path::new("/tmp/crust/");
    let mut tmp_pathbuf = PathBuf::new();
    tmp_pathbuf.push("/tmp/crust");
    tmp_pathbuf.push(datetime.to_string());
    println!("{}", tmp_pathbuf.display());
    let mut path_vec: Vec<PathBuf> = Vec::new();

    for entry in fs_entries.iter() {
        path_vec.push(format_fs_path(entry))
    }
    let mut dir_builder = DirBuilder::new();
    dir_builder.recursive(true);
    for path in path_vec { // TODO: Recursive traverse
        let tmp_final_path = tmp_pathbuf.join(path.strip_prefix("/").unwrap());
        if path.is_dir() {
            dir_builder.create(&tmp_final_path).unwrap();
            for entry in read_dir(path).unwrap() {
                match entry {
                    Ok(e) => {
                        let final_path = tmp_pathbuf.join(e.path().strip_prefix("/").unwrap());
                        println!("{}", final_path.display());
                        copy(e.path(), final_path).unwrap();
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        } else if path.is_file() {
            let final_path = tmp_pathbuf.join(path.strip_prefix("/").unwrap());
            let mut dir_path = final_path.clone();
            dir_path.pop();
            dir_builder.create(dir_path).unwrap();
            if let Err(e) = copy(&path, final_path) {
                println!("{}: {}", path.display(), e);
            }
        }
    }
}

fn format_fs_path(dir_string: &toml::Value) -> PathBuf {
    let mut home_dir = home_dir().unwrap_or_default();
    let home_dir_str = home_dir.as_mut_os_str().to_str().unwrap_or_default();
    let mut mut_dir_string = String::from_str(dir_string.as_str().unwrap()).unwrap_or_default();
    mut_dir_string = mut_dir_string.replace("~", home_dir_str);
    PathBuf::from(mut_dir_string)
}