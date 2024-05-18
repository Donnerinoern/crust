use tar::Builder;
use zstd::stream::copy_encode;
use std::{fs::File, io::{BufReader, BufWriter}, path::Path};

pub fn archive(tmp_path: &Path) {
    let mut pathbuf = tmp_path.to_path_buf();
    pathbuf.set_extension("tar");
    let tar_file = File::create_new(&pathbuf).unwrap();
    let mut tar_builder = Builder::new(&tar_file);
    tar_builder.append_dir_all(".", tmp_path).unwrap();
    tar_builder.finish().unwrap();
    compress(&pathbuf);
}

fn compress(path: &Path) {
    let archive = File::open(path).unwrap();
    let mut pathbuf = path.to_path_buf();
    pathbuf.set_extension("tar.zst");
    let compressed_archive = File::create_new(pathbuf).unwrap();
    let bufreader = BufReader::new(archive);
    let bufwriter = BufWriter::new(compressed_archive);
    copy_encode(bufreader, bufwriter, 0).unwrap();
}
