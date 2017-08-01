extern crate mio;

use mio::net::TcpListener;
use mio::net::TcpStream;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::thread;

struct GameState {
    world: Vec<Room>,
    time: u64,
    wizlock: bool,
    clients: Vec<Client>,
    slow_death: bool,
    shutdown: bool,
    reboot: bool,
}

struct Room {
    number: u32,
    zone: u32,
    sector_type: SectorType,
    name: String,
    description: String,
    extra: ExtraDescription,
    directions: HashMap<Direction, RoomDirection>,
    dark: bool,
    no_mob: bool,
    indoors: bool,
}

struct RoomDirection {
    description: String,        // When look DIR.
    keyword: String,            // for open/close
    is_door: bool,
    closed: bool,
    locked: bool,
    pickproof: bool,
}

struct ExtraDescription {
    keyword: String,
    description: String,
}

enum SectorType {
    Inside,
    City,
    Field,
    Forest,
    Hills,
    Mountains,
    Swimming,
    Unswimmable,
}

enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

struct Client {
    stream: TcpStream,
    output: VecDeque<String>,
    input: VecDeque<String>,
    conn: Connection,
}

enum Connection {
    Initial,
    Creating,
    LoggingIn(String),
    //Connected(Character),
}

fn build_rooms() -> Vec<Room> {
    Vec::new()
}

fn main() {

    let game = GameState { world: build_rooms() };

    println!("Creating server");

    // let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 9000;
    let host = "127.0.0.1";

    let socket = SocketAddr::new(IpAddr::V4(host.parse().unwrap()), port);
    let mut listener = TcpListener::bind(&socket).unwrap();
    println!("Listener bound, ready to accept connections");
    while !game.shutdown {
        for stream in listener.incoming() {
            thread::spawn(|| { handle_client(stream.unwrap()); });
        }
    }
}

fn handle_client(mut stream : TcpStream) {
    let _ = stream.write(b"Welcome to Colony!\r\n");
}
