use crossbeam_channel::{Sender, Receiver};
use serde_json::json;
use log::error;
use std::error::Error;
use std::net::{SocketAddr, UdpSocket, ToSocketAddrs};
use std::sync::Arc;

#[derive(PartialEq, Debug, Clone)]
pub struct ClientRPC {
    pub command: String,
    pub src: SocketAddr,
}

pub struct ClientServer {
    socket: UdpSocket,
    pub addr: SocketAddr,
    peer: Vec<SocketAddr>,
}

pub struct Server {
    pub CS: Arc<ClientServer>,
    pub notifier: Option<Sender<ClientRPC>>,
    pub receiver: Option<Receiver<ClientRPC>>,
}

impl ClientServer {
    pub fn new(addr: SocketAddr, peer: Vec<SocketAddr>) -> Result<ClientServer, Box<dyn Error>> {
        let socket = UdpSocket::bind(addr)?;
        Ok(ClientServer {
            socket,
            addr,
            peer,
        })
    }
    // Send response to client
    pub fn response_client(
        &self,
        client_addr: SocketAddr,
        msg_to_send: String,
    ) -> Result<(), Box<dyn Error>> {
        //recv_node: host, port
        let buffer = msg_to_send.as_bytes();
        self.socket.send_to(&buffer, client_addr)?;
        Ok(())
    }
    pub fn start_listener(&self, client_notifier: Sender<ClientRPC>) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 65535];
        loop {
            let (amt, src) = self.socket.recv_from(&mut buffer).unwrap();
            if let Ok(msg_content) = String::from_utf8(buffer[..amt].to_vec()) {
                let msg_received = ClientRPC{
                    command: msg_content,
                    src: src,
                };
                client_notifier.send(msg_received)?;
            }
        }
    }
}