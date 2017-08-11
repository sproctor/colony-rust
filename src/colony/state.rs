extern crate mio;

use mio::tcp::TcpStream;
use std::io::Read;
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

    pub fn get_clients(&self) -> &Vec<Client> {
        &self.clients
    }
}

pub struct Client {
    stream: TcpStream,
    output: VecDeque<String>,
    input: VecDeque<String>,
    buffer: String,
    state: ClientState,
}

impl Client {
    fn new(stream: TcpStream) -> Client {
        Client {
            stream: stream,
            output: VecDeque::new(),
            input: VecDeque::new(),
            buffer: String::new(),
            state: ClientState::Initial,
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        let _ = self.stream.write(buf);
    }

    pub fn read(&mut self) {
        let mut buffer = String::new();
        match self.stream.read_to_string(&mut buffer) {
            Ok(_) => { self.buffer += &buffer; }
            Error => {}
        }
    }

    pub fn get_command(&mut self) -> Option<String> {
        if self.buffer.len() > 0 {
            let result = self.buffer;
            self.buffer = String::new();
            return Some(result);
        } else {
            return None;
        }
    }
}

enum ClientState {
    Initial,
    Creating,
    LoggingIn(String),
    //Connected(Character),
}

