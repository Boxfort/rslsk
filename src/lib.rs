use std::net::{TcpStream};
use std::io::{Error, Write, Read};
use std::thread;
use protocol::message::{Message};
use protocol::input_message::InputMessage;
use std::sync::mpsc::{channel, Sender};
use server::Server;
use crate::server::Listener;

mod protocol;
mod server;

pub struct Slsk {
    username: &'static str,
    password: &'static str,
    server_out: Sender<Box<dyn Message>>,
}

impl Slsk {
    pub fn connect(server: &'static str, port: u16, username: &'static str, password: &'static str) -> Result<Self, Error> {
        let address = format!("{}:{}", server, port);

        println!("{}", address);
        match TcpStream::connect(address) {
            Ok(mut serverStream) => {
                let server = Server::new(serverStream.try_clone().unwrap());

                Result::Ok(
                    Slsk {
                        username,
                        password,
                        server_out: server.out,
                    }
                )

            },
            Err(error) => {
                println!("{}", error.to_string());
                Result::Err(error)
            }
        }
    }

    pub fn login(&self) -> Result<(), Error> {
        self.server_out.send(Message::login_request(self.username, self.password));
        Result::Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn none() {

    }
}
