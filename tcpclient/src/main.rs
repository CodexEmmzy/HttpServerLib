use std::{net::TcpStream, io::{Write, Read}, str::from_utf8};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0 as u8; 5];

    stream.read(&mut buffer).unwrap();
    println!("Got response from server: {:?}", from_utf8(&buffer).unwrap());
}