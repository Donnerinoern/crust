use std::{fs::{copy, read_dir, DirEntry, File}, io::{BufReader, BufWriter}, path::{Path, PathBuf}, str::FromStr};
use zstd::stream::copy_decode;
use tar::Archive;

pub fn decompress(path_str: &str, tmp_path: &Path) {
    let compressed_archive= File::open(path_str);
    match compressed_archive {
        Err(e) => {
            println!("{}", e);
        }
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut archive_path_string = String::from_str(path_str).unwrap();
            archive_path_string.remove_matches(".zst");
            let archive_path = PathBuf::from_str(archive_path_string.as_str()).unwrap();
            let archive = File::create_new(&archive_path).unwrap();
            let writer = BufWriter::new(archive);
            copy_decode(reader, writer).unwrap();
            unpack(&archive_path, tmp_path);
        }
    }
}

fn unpack(archive_path: &Path, tmp_path: &Path) {
    let mut archive = Archive::new(File::open(archive_path).unwrap());
    let mut tmp_pathbuf = PathBuf::from(tmp_path);
    tmp_pathbuf.push("unpacked");
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
                    copy(e.path(), dest_string).unwrap();
                }
            }
        }
    }
}
