use std::{io, net};

use chunked::{ChunkSize, ChunkedMessage};

pub struct UdpDestination {
    socket: net::UdpSocket,
    destination: net::SocketAddr,
    chunk_size: ChunkSize,
}

impl UdpDestination {
    pub fn new<T: net::ToSocketAddrs>(
        destination: T,
        chunk_size: ChunkSize,
    ) -> Result<Self, io::Error> {
        let destination = destination
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid destination"))?;

        let local = match destination {
            net::SocketAddr::V4(_) => "0.0.0.0:0",
            net::SocketAddr::V6(_) => "[::]:0",
        };

        let socket = net::UdpSocket::bind(local)?;

        Ok(UdpDestination {
            socket,
            destination,
            chunk_size,
        })
    }

    pub fn log(&self, message: Vec<u8>) -> Result<(), io::Error> {
        let chunked_message = ChunkedMessage::new(self.chunk_size, message)?;

        let sent_bytes: usize = chunked_message
            .iter()
            .map(|chunk| self.socket.send_to(&chunk, self.destination).unwrap_or(0))
            .sum();

        if sent_bytes != chunked_message.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Invalid number of bytes sent",
            ));
        }

        Ok(())
    }
}
