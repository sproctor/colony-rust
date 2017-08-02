extern crate mio;

use mio::tcp::TcpStream;
use std::io::Write;
use std::collections::HashMap;
use std::collections::VecDeque;

use colony::room::Room;

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

    pub fn handle_client(&mut self, mut stream: TcpStream) {
        let _ = stream.write(b"Welcome to Colony!\r\n");
        let client = Client::new(stream);
        self.clients.push(client);
    }
}

struct Client {
    stream: TcpStream,
    output: VecDeque<String>,
    input: VecDeque<String>,
    buffer: String,
    conn: Connection,
}

impl Client {
    fn new(stream: TcpStream) -> Client {
        Client {
            stream: stream,
            output: VecDeque::new(),
            input: VecDeque::new(),
            buffer: String::new(),
            conn: Connection::Initial,
        }
    }

    fn write(&mut self, buf: &[u8]) {
        //let _ = self.stream.write(buf);
    }
}

enum Connection {
    Initial,
    Creating,
    LoggingIn(String),
    //Connected(Character),
}

