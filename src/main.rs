use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    println!("Creating server");

    // let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 9000;
    let host = "127.0.0.1";

    let listener = TcpListener::bind((host, port)).unwrap();
    println!("Listener bound, ready to accept connections");
    for stream in listener.incoming() {
    	thread::spawn(|| { handle_client(stream.unwrap()); });
    }
}

fn handle_client(mut stream : TcpStream) {
    let _ = stream.write(b"Welcome to Colony!\r\n");
}
