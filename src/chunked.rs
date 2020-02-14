use rand::{thread_rng, RngCore};
use std::{cmp, io};

const CHUNK_SIZE_LAN: u16 = 8154;
const CHUNK_SIZE_WAN: u16 = 1420;
static MAGIC_BYTES: &[u8; 2] = b"\x1e\x0f";

/// Overhead per chunk is 12 bytes: magic(2) + id(8) + pos(1) + total (1)
const CHUNK_OVERHEAD: u8 = 12;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ChunkSize {
    LAN,
    WAN,
    Custom(u16),
}

impl ChunkSize {
    pub fn size(self) -> u16 {
        match self {
            ChunkSize::LAN => CHUNK_SIZE_LAN,
            ChunkSize::WAN => CHUNK_SIZE_WAN,
            ChunkSize::Custom(size) => size,
        }
    }
}

pub struct ChunkedMessage {
    chunk_size: ChunkSize,
    payload: Vec<u8>,
    num_chunks: u8,
    id: ChunkedMessageId,
}

impl ChunkedMessage {
    /// Several sanity checks are performed on construction:
    /// - chunk_size must be greater than 0
    /// - GELF allows for a maximum of 128 chunks per message
    pub fn new(chunk_size: ChunkSize, payload: Vec<u8>) -> Result<Self, io::Error> {
        if chunk_size.size() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Chunk size cannot be zero",
            ));
        }

        let size = chunk_size.size() as u64;
        let num_chunks = ((payload.len() as u64 + size as u64 - 1) / size) as u8;

        if num_chunks > 128 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Number of chunks exceeds 128",
            ));
        }

        let id = ChunkedMessageId::random();

        Ok(ChunkedMessage {
            chunk_size,
            payload,
            num_chunks,
            id,
        })
    }

    /// Return the byte-length of the chunked message inclduing all overhead
    pub fn len(&self) -> usize {
        if self.num_chunks > 1 {
            self.payload.len() + self.num_chunks as usize * CHUNK_OVERHEAD as usize
        } else {
            self.payload.len()
        }
    }

    /// Return an iterator over all chunks of the message
    pub fn iter(&self) -> ChunkedMessageIterator {
        ChunkedMessageIterator::new(self)
    }
}

pub struct ChunkedMessageIterator<'a> {
    chunk_num: u8,
    message: &'a ChunkedMessage,
}

impl<'a> ChunkedMessageIterator<'a> {
    fn new(msg: &'a ChunkedMessage) -> ChunkedMessageIterator {
        ChunkedMessageIterator {
            chunk_num: 0,
            message: msg,
        }
    }
}

impl<'a> Iterator for ChunkedMessageIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        if self.chunk_num >= self.message.num_chunks {
            return None;
        }

        let mut chunk = Vec::new();

        // Set the chunks boundaries
        let chunk_size = self.message.chunk_size.size();
        let slice_start = (self.chunk_num as u32 * chunk_size as u32) as usize;
        let slice_end = cmp::min(
            slice_start + chunk_size as usize,
            self.message.payload.len(),
        );

        if self.message.num_chunks > 1 {
            // Chunk binary layout:
            //  2 bytes (magic bytes)
            //  8 bytes (message id)
            //  1 byte  (chunk number)
            //  1 byte  (total amount of chunks in this message)
            //  n bytes (chunk payload)
            chunk.extend(MAGIC_BYTES.iter());
            chunk.extend(self.message.id.as_bytes());
            chunk.push(self.chunk_num);
            chunk.push(self.message.num_chunks);
        }
        chunk.extend(self.message.payload[slice_start..slice_end].iter());

        self.chunk_num += 1;

        Some(chunk)
    }
}

/// The representation of a chunked message id
///
/// Every chunked message requires an ID which consists of 8 bytes. This is the same
/// as an 64bit integer. This struct provides some convenience functions on this type.
struct ChunkedMessageId([u8; 8]);

#[allow(dead_code)]
impl<'a> ChunkedMessageId {
    fn random() -> ChunkedMessageId {
        let mut bytes = [0; 8];
        thread_rng().fill_bytes(&mut bytes);

        ChunkedMessageId::from_bytes(bytes)
    }

    fn from_int(mut id: u64) -> ChunkedMessageId {
        let mut bytes = [0; 8];
        for i in 0..8 {
            bytes[7 - i] = (id & 0xff) as u8;
            id >>= 8;
        }

        ChunkedMessageId(bytes)
    }

    fn from_bytes(bytes: [u8; 8]) -> ChunkedMessageId {
        ChunkedMessageId(bytes)
    }

    fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }

    fn to_int(&self) -> u64 {
        self.0.iter().fold(0_u64, |id, &i| id << 8 | i as u64)
    }
}
