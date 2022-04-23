use std::net::TcpListener;
use std::io::{Read, Write};
fn main() {
    //Initialize a socket server to bind for an IP address 127.0.0.1(localhost) and port 9000
    let connection_listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    println!("TCP Server is listening on port 9000");
    //The TCP socket server is listening for incoming connections
    for stream in connection_listener.incoming() {
        //The stream is made mutable so I can read and write to it.
        let mut stream = stream.unwrap();
        println!("TCP Server Connection is established");
        //Buffer size is taken for big amount of bytes for testing amount of info handled in one request
        let mut buffer = [0;4096];
        //Reading from incoming stream
        stream.read(&mut buffer).unwrap();
        //Echo back whatever is received, to the client on the same session
        stream.write(&mut buffer).unwrap();
    }
}