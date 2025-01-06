use crate::protocol::Message;
use crate::sound_scheduler::SoundScheduler;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct ClientHandler;

impl ClientHandler {
    fn handle_client(mut stream: TcpStream) {
        println!("Handling client: {:?}", stream);
        let mut buf: Vec<u8> = Vec::new();
        stream
            .read_to_end(buf.as_mut())
            .expect("Could not read from stream");
        let message = bincode::deserialize::<Message>(&buf).expect("Could not deserialize message");
        match message {
            Message::PlaySound(play_sound) => {
                SoundScheduler::handle_scheduled_sound(play_sound);
            }
        }
    }

    pub fn listen() {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("Could not bind to address");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    ClientHandler::handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
