use tar::Builder;
use zstd::stream::copy_encode;
use std::{env, fs::File, io::{BufReader, BufWriter}, path::Path};

pub fn archive(tmp_path: &Path) { // TODO: Try to do in memory
    let mut pathbuf = tmp_path.to_path_buf();
    pathbuf.set_extension("tar");
    let tar_file = File::create_new(&pathbuf).unwrap();
    let mut tar_builder = Builder::new(&tar_file);
    tar_builder.append_dir_all(".", tmp_path).unwrap();
    tar_builder.finish().unwrap();
    compress(&pathbuf);
}

fn compress(path: &Path) { // TODO: Try to do in memory
    let archive = File::open(path).unwrap();
    let file_stem = path.file_stem().unwrap();
    let mut current_path= env::current_dir().unwrap();
    current_path.push(file_stem);
    current_path.set_extension("tar.zst");
    let compressed_archive = File::create_new(current_path).unwrap();
    let bufreader = BufReader::new(archive);
    let bufwriter = BufWriter::new(compressed_archive);
    copy_encode(bufreader, bufwriter, 0).unwrap();
}
