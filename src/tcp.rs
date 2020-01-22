use std::{io, io::Write, net, sync::RwLock};

use super::Destination;

pub struct TcpDestination {
    socket: RwLock<net::TcpStream>,
}

impl TcpDestination {
    pub fn new<T: net::ToSocketAddrs>(destination: T) -> Result<Self, io::Error> {
        let destination = destination
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid destination"))?;

        let socket = net::TcpStream::connect(destination)?;

        Ok(Self {
            socket: RwLock::new(socket),
        })
    }
}

impl Destination for TcpDestination {
    fn log(&self, message: Vec<u8>) -> Result<(), io::Error> {
        // At the current time, GELF TCP only supports uncompressed and non-chunked payloads.
        // https://docs.graylog.org/en/3.0/pages/gelf.html#gelf-via-tcp

        // Each message needs to be delimited with a null byte (\0) when sent in the same TCP connection.
        let mut message = message;
        message.push(0);

        let mut socket = self.socket.write().expect("Poisoned socket lock");
        let sent_bytes = socket.write(&message).unwrap_or(0);

        if sent_bytes != message.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Invalid number of bytes sent",
            ));
        }

        Ok(())
    }
}
