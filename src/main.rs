extern crate mio;

mod colony;

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::{TcpListener, TcpStream};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use colony::state::GameState;

fn main() {

    let mut game = GameState::new();

    println!("Creating server");

    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 9000;
    // let host = "127.0.0.1";

    // start listening
    let addr = SocketAddr::new(IpAddr::V4(ip), port);
    let listener = TcpListener::bind(&addr).unwrap();
    let poll = Poll::new().unwrap();
    poll.register(&listener, Token(0), Ready::readable(), PollOpt::edge()).unwrap();
    let mut events = Events::with_capacity(128);

    println!("Listener bound, ready to accept connections");
    while !game.is_shutdown() {
        poll.poll(&mut events, None).unwrap();

        for _ in events.iter() {
            let (stream, _addr) = listener.accept().unwrap();
            handle_client(stream);
            game.shutdown();
        }
    }

    println!("Shutting down server!");
}

fn handle_client(mut stream : TcpStream) {
    let _ = stream.write(b"Welcome to Colony!\r\n");
}
