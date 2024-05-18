use std::{fs::{copy, create_dir, create_dir_all, read_dir, DirEntry, File}, io::{BufReader, BufWriter}, path::{Path, PathBuf}};
use zstd::stream::copy_decode;
use tar::Archive;

pub fn decompress(tmp_path: &Path, input_path: &Path) {
    let compressed_archive= File::open(input_path);
    match compressed_archive {
        Err(e) => {
            println!("{}", e);
        }
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut archive_path = PathBuf::from(tmp_path);
            archive_path.set_extension("tar");
            create_dir("/tmp/crust").unwrap();
            let archive_file = File::create_new(&archive_path).unwrap();
            let writer = BufWriter::new(archive_file);
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
                    let mut dest_path = PathBuf::from(&dest_string);
                    dest_path.pop();
                    create_dir_all(dest_path).unwrap();
                    copy(e.path(), dest_string).unwrap();
                }
            }
        }
    }
}
