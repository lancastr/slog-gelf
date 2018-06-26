use std::{
    io,
    net,
};

//use message::{MessageCompression, ChunkSize};

pub struct UdpDestination {
    socket: net::UdpSocket,
    destination: net::SocketAddr,
//    chunk_size: ChunkSize,
//    compression: MessageCompression,
}

impl UdpDestination {
    pub fn new<T: net::ToSocketAddrs>(destination: T) -> Result<Self, io::Error> {
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
//            chunk_size: chunk_size,
//            compression: MessageCompression::default(),
        })
    }

//    /// Return the current set compression algorithm
//    pub fn compression(&self) -> MessageCompression {
//        self.compression
//    }
//
//    /// Set the compression algorithm
//    pub fn set_compression(&mut self, compression: MessageCompression) -> &mut Self {
//        self.compression = compression;
//        self
//    }

    pub fn log(&self, msg: &str) -> Result<(), io::Error> {
//        let chunked_msg = msg.to_chunked_message(self.chunk_size, self.compression)?;
//        let chunked_msg_size = chunked_msg.len();
//        let sent_bytes = chunked_msg.iter()
//            .map(|chunk| match self.socket.send_to(&chunk, self.destination) {
//                Err(_) => 0,
//                Ok(size) => size,
//            })
//            .fold(0_u64, |carry, size| carry + size as u64);
//
//        if sent_bytes != chunked_msg_size {
//            bail!(ErrorKind::LogTransmitFailed);
//        }
        self.socket.send_to(msg.as_bytes(), self.destination)?;

        Ok(())
    }
}