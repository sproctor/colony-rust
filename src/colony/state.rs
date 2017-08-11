extern crate mio;

use mio::net::TcpStream;
use mio::Token;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::replace;
use std::rc::Rc;

use colony::room::Room;

pub struct GameState {
    world: Vec<Room>,
    time: u64,
    wizlock: bool,
    clients: Vec<Rc<Client>>,
    slow_death: bool,
    shutdown: bool,
    reboot: bool,
    next_socket_index: usize,
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
            next_socket_index: 100,
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
        let client = Rc::new(Client::new(stream, self.get_token()));
        self.clients.push(client);
    }

    // FIXME: &mut self seems wrong here
    pub fn get_clients(&self) -> [Rc<Client>] {
        self.clients.as_mut()
    }

    pub fn find_client_by_token(&mut self, token: Token) -> Option<&mut Client> {
        for client in self.clients.iter_mut() {
            // If stream is client.stream
            if client.get_token() == token {
                return Some(&mut client);
            }
        }
        None
    }

    pub fn get_token(&mut self) -> Token {
        let token = Token(self.next_socket_index);
        self.next_socket_index += 1;
        token
    }
}

pub struct Client<'a> {
    token: Token,
    stream: TcpStream,
    output: VecDeque<String>,
    input: VecDeque<String>,
    buffer: String,
    state: ClientState,
}

impl<'a> Client<'a> {
    fn new(stream: TcpStream, token: Token) -> Client<'a> {
        Client {
            stream: stream,
            output: VecDeque::new(),
            input: VecDeque::new(),
            buffer: String::new(),
            state: ClientState::Initial,
            token: token,
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        let _ = self.stream.write(buf);
    }

    pub fn read(&mut self) {
        let mut buffer = String::new();
        match self.stream.read_to_string(&mut buffer) {
            Ok(_) => { self.buffer += &buffer; }
            Err(_) => {}
        }
    }

    pub fn get_command(&mut self) -> Option<String> {
        if self.buffer.len() > 0 {
            Some(replace(&mut self.buffer, String::new()))
        } else {
            None
        }
    }

    pub fn get_token(&self) -> Token {
        self.token
    }
}

enum ClientState {
    Initial,
    Creating,
    LoggingIn(String),
    //Connected(Character),
}

