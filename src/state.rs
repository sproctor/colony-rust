extern crate mio;

use mio::net::TcpStream;
use std::collections::HashMap;
use std::collections::VecDeque;

use room::Room;

pub struct GameState {
    world: Vec<Room>,
    time: u64,
    wizlock: bool,
    clients: Vec<Client>,
    slow_death: bool,
    shutdown: bool,
    reboot: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            world: Room::build_rooms(),
            time: 0,
            wizlock: false,
            clients: Vec::new(),
            slow_death: false,
            shutdown: false,
            reboot: false,
        }
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }
}

struct Client {
    stream: TcpStream,
    output: VecDeque<String>,
    input: VecDeque<String>,
    buffer: String,
    conn: Connection,
}

enum Connection {
    Initial,
    Creating,
    LoggingIn(String),
    //Connected(Character),
}

