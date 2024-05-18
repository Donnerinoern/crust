use tar::Builder;
use zstd::stream::copy_encode;
use std::{collections::VecDeque, env, fs::File, path::Path};

pub fn archive(tmp_path: &Path) {
    let byte_vec: VecDeque<u8> = VecDeque::new();
    let mut builder = Builder::new(byte_vec);
    builder.append_dir_all(".", tmp_path).unwrap();
    builder.finish().unwrap();
    compress(builder.get_mut());
}

fn compress(byte_vec: &mut VecDeque<u8>) {
    let mut path = env::current_dir().unwrap();
    path.push("archive");
    path.set_extension("tar.zst");
    let file = File::create_new(path).unwrap();
    copy_encode(byte_vec, file, 0).unwrap();
}
