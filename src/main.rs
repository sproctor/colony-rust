extern crate mio;

mod colony;

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};
use std::thread::sleep;

use colony::state::GameState;

fn main() {

    let mut game = GameState::new();

    println!("Creating server");

    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 9000;
    let pulse_duration = Duration::new(3, 0);
    // let host = "127.0.0.1";

    let boot_time = Instant::now();
    let mut expected_time = boot_time.clone();

    // start listening
    // TODO: move this stuff into GameState
    let addr = SocketAddr::new(IpAddr::V4(ip), port);
    let listener = TcpListener::bind(&addr).unwrap();
    let incoming = Poll::new().unwrap();
    incoming.register(&listener, Token(0), Ready::readable(), PollOpt::edge()).unwrap();
    let mut events = Events::with_capacity(128);
    let client_streams = Poll::new().unwrap();

    println!("Listener bound, ready to accept connections");
    while !game.is_shutdown() {
        println!("Pulsing");

        // sleep until the next pulse should start
        expected_time += pulse_duration;
        if expected_time < Instant::now() {
            println!("We're lagging pretty bad.")
        } else {
            sleep(expected_time - Instant::now());
        }

        incoming.poll(&mut events, Some(Duration::from_secs(0))).unwrap();

        for _ in events.iter() {
            let (stream, _addr) = listener.accept().unwrap();
            client_streams.register(&stream, Token(0), Ready::readable(), PollOpt::edge());
            game.handle_client(stream);
        }

        client_streams.poll(&mut events, Some(Duration::from_secs(0))).unwrap();
        for client in game.get_clients() {
            client.read();
            match client.get_command() {
                Some(command) => println!("client buffer: {}", command);
                None => println!("command not yet ready");
            }
            
        }
    }

    println!("Shutting down server!");
}