use std::{fs::File, io::{BufReader, BufWriter, Read, Write}, net::{SocketAddrV4, TcpListener, TcpStream}, path::Path};
use crate::Network;

pub fn create_tcp_listener(network_config: &Network) {
    let key = generate_key();
    println!("Key: {}", key);
    let socket_addr = SocketAddrV4::new(network_config.ip_addr.unwrap(), network_config.port.unwrap());
    println!("Listening on: {}", socket_addr);
    let listener = TcpListener::bind(socket_addr).unwrap();
    for stream in listener.into_incoming().filter_map(Result::ok) {
        handle_client(stream);
    }
}

fn handle_client(stream: TcpStream) {
    let mut buf = Vec::new();
    let mut reader = BufReader::new(stream);
    reader.read_to_end(&mut buf).unwrap();
    let file = File::create_new("network_archive.tar.zst").unwrap();
    let mut writer = BufWriter::new(file);
    match writer.write_all(&buf) {
        Ok(_) => println!("Archive successfully recievied."),
        Err(e) => println!("{}", e)
    }
}

pub fn create_tcp_stream(network_config: &Network, path: &Path) {
    let socket_addr = SocketAddrV4::new(network_config.ip_addr.unwrap(), network_config.port.unwrap());
    let stream = TcpStream::connect(socket_addr);
    match stream {
        Ok(s) => {
            let mut buf = Vec::new();
            let mut writer = BufWriter::new(s);
            let mut reader = BufReader::new(File::open(path).unwrap());
            reader.read_to_end(&mut buf).unwrap();
            match writer.write_all(&buf) {
                Ok(_) => println!("Successfully sent archive: '{}'", path.display()),
                Err(e) => println!("{}", e)
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn generate_key() -> u16 {
    rand::random()
}
