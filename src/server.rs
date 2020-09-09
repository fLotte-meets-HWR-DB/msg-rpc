use std::net::{TcpListener, TcpStream};
use std::io::{Read, ErrorKind, Write};
use std::io;
use byteorder::{BigEndian, ByteOrder};
use crate::message::Message;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crossbeam_utils::sync::WaitGroup;
use std::mem;

const BUF_SIZE: usize = 512;

#[derive(Clone, Debug)]
pub struct MessageHandler {
    message: Message,
    pub response: Option<Message>,
    pub wg: WaitGroup,
}

impl MessageHandler {
    pub fn done(&mut self, response: Message) {
        self.response = Some(response);
        self.wg = WaitGroup::new();
    }
}

#[derive( Debug)]
pub struct RpcServer {
    address: String,
    pub receiver: Arc<Mutex<Receiver<Arc<Mutex<MessageHandler>>>>>,
    sender: Sender<Arc<Mutex<MessageHandler>>>
}

impl RpcServer {
    /// Starts the RPC server
    pub fn start(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        for stream in listener.incoming() {
            match stream {
                Ok(s) => if let Err(e) = self.handle_message(s) {
                    log::trace!("Error handling message {}", e.to_string())
                },
                Err(e) => log::trace!("TCP Error {}", e.to_string())
            }
        }

        Ok(())
    }

    /// Handles a message
    fn handle_message(&mut self, mut incoming: TcpStream) -> io::Result<()> {
        let mut length_raw = [0u8; 4];
        incoming.read_exact(&mut length_raw)?;
        let length = BigEndian::read_u32(&length_raw);
        let mut data = Vec::new();
        data.append(&mut length_raw.to_vec());

        for _ in 0..(length as f32 / BUF_SIZE as f32).ceil() as usize {
            let mut buf = [0u8; BUF_SIZE];
            incoming.read(&mut buf)?;
            data.append(&mut buf.to_vec())
        }

        let message = Message::from_bytes(&data).map_err(|_|io::Error::from(ErrorKind::InvalidData))?;
        let wg = WaitGroup::new();

        let handler = Arc::new(Mutex::new(MessageHandler {
            message,
            wg: WaitGroup::clone(&wg),
            response: None,
        }));
        self.sender.send(Arc::clone(&handler)).unwrap();
        wg.wait();
        if let Some(response) = mem::replace(&mut handler.lock().unwrap().response, None) {
            incoming.write(&response.to_bytes())?;
        }

        Ok(())
    }
}