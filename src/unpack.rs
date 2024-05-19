use std::{collections::VecDeque, fs::{copy, create_dir_all, read_dir, DirEntry, File}, io::BufReader, path::{Path, PathBuf}};
use zstd::stream::copy_decode;
use tar::Archive;
use crate::{Config, filesystem};

pub fn decompress(config: &Config) {
    let compressed_archive = File::open(config.file_path.as_ref().unwrap());
    match compressed_archive {
        Err(e) => {
            println!("{}", e);
        }
        Ok(f) => {
            let reader = BufReader::new(f);
            filesystem::create_tmp_dir();
            let mut bytes_vec: VecDeque<u8> = VecDeque::new();
            copy_decode(reader, &mut bytes_vec).unwrap();
            unpack(&config.tmp_path, bytes_vec);
        }
    }
}

fn unpack(tmp_path: &Path, bytes_vec: VecDeque<u8>) {
    let mut tmp_pathbuf = PathBuf::from(tmp_path);
    tmp_pathbuf.push("unpacked");
    let mut archive = Archive::new(bytes_vec);
    archive.unpack(&tmp_pathbuf).unwrap();
    copy_or_overwrite(&tmp_pathbuf);
}

fn copy_or_overwrite(path: &Path) {
    let entries = read_dir(path).unwrap();
    for entry in entries {
        match entry {
            Err(e) => {
                println!("{}", e);
            }
            Ok(e) => {
                if e.file_type().unwrap().is_dir() {
                    traverse_and_copy(e, path);
                } else {
                    // e is envvars.crust
                }
            }
        }
    }
}

fn traverse_and_copy(dir: DirEntry, path: &Path) {
    for entry in read_dir(dir.path()).unwrap() {
        match entry {
            Err(e) => {
                println!("{}", e);
            }
            Ok(e) => {
                if e.file_type().unwrap().is_dir() {
                    traverse_and_copy(e, path);
                } else if e.file_type().unwrap().is_file() {
                    let mut dest_string = String::from(e.path().to_str().unwrap());
                    dest_string.remove_matches(path.to_str().unwrap());
                    let mut dest_path = PathBuf::from(&dest_string);
                    dest_path.pop();
                    create_dir_all(dest_path).unwrap();
                    copy(e.path(), dest_string).unwrap();
                }
            }
        }
    }
}
