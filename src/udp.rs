use std::{
    io,
    net,
};

use chunked::{ChunkedMessage, ChunkSize};

pub struct UdpDestination {
    socket: net::UdpSocket,
    destination: net::SocketAddr,
    chunk_size: ChunkSize,
}

impl UdpDestination {
    pub fn new<T: net::ToSocketAddrs>(destination: T, chunk_size: ChunkSize) -> Result<Self, io::Error> {
        let destination = destination
            .to_socket_addrs()?
            .nth(0)
            .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid destination"))?;

        let local = match destination {
            net::SocketAddr::V4(_) => "0.0.0.0:0",
            net::SocketAddr::V6(_) => "[::]:0",
        };

        let socket = net::UdpSocket::bind(local)?;
        socket.set_nonblocking(true)?;

        Ok(UdpDestination {
            socket,
            destination,
            chunk_size,
        })
    }

    pub fn log(&self, message: &str) -> Result<(), io::Error> {
        let chunked_message = ChunkedMessage::new(
            self.chunk_size,
            message.as_bytes().to_vec()
        )?;

        let sent_bytes = chunked_message
            .iter()
            .map(|chunk| match self.socket.send_to(&chunk, self.destination) {
                Err(_) => 0,
                Ok(size) => size,
            })
            .fold(0_u64, |carry, size| carry + size as u64);

        if sent_bytes != chunked_message.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Invalid number of bytes sent"));
        }

        Ok(())
    }
}