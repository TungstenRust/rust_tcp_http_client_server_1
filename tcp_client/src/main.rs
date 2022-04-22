use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:9000").unwrap();
    //Write a "Hello" message to the TCP server connection
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    //Read the bytes received from server
    stream.read(&mut buffer).unwrap();
    println!(
        //Prints out whatever is received from the server.
        // The server sends raw bytes and I have to convert it into
        // UTF-8 str type to print it to terminal.
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}