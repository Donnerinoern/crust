use std::{path::Path, time::{Instant, SystemTime}};

pub fn handle_directories(directories: toml::value::Array) {
    let tmp_path: &Path = Path::new("/tmp/crust/");
    let time_now: SystemTime = SystemTime::now();
    let time_instant = Instant::now();
    println!("{:?}", time_now);
    println!("{:?}", time_instant);
}
