mod chunked;
mod level;
mod message;
mod udp;

extern crate chrono;
extern crate flate2;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate slog;

use slog::{Drain, Key, OwnedKVList, Record, KV};
use std::io;

use chunked::ChunkSize;
use message::Message;
use udp::UdpDestination;

static VERSION: &str = "1.1";

pub struct Gelf<D: Destination> {
    source: String,
    destination: D,
}

pub trait Destination {
    fn log(&self, message: Vec<u8>) -> Result<(), io::Error>;
}

impl Gelf<UdpDestination> {
    pub fn with_udp(source: &str, destination: &str) -> Result<Self, io::Error> {
        let destination = UdpDestination::new(destination, ChunkSize::LAN)?;

        Ok(Gelf {
            source: source.to_owned(),
            destination,
        })
    }

    #[deprecated(since = "0.1.3", note="Use `Gelf::with_udp` method instead.")]
    pub fn new(source: &str, destination: &str) -> Result<Self, io::Error> {
        Self::with_udp(source, destination)
    }
}

pub struct KeyValueList(pub Vec<(Key, String)>);

impl slog::Serializer for KeyValueList {
    fn emit_arguments(&mut self, key: Key, val: &std::fmt::Arguments) -> slog::Result {
        self.0.push((key, format!("{}", val)));
        Ok(())
    }
}

impl<D: Destination> Drain for Gelf<D> {
    type Ok = ();
    type Err = io::Error;

    fn log(&self, record: &Record, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        let mut additional = KeyValueList(Vec::with_capacity(16));
        record.kv().serialize(record, &mut additional)?;
        values.serialize(record, &mut additional)?;

        let message = Message {
            version: VERSION,
            host: &self.source,
            short_message: record.msg().to_string(),
            full_message: None,
            timestamp: Some(timestamp()),
            level: Some(record.level().into()),
            module: Some(record.location().module),
            file: Some(record.location().file),
            line: Some(record.location().line),
            column: None,
            additional: additional.0,
        };

        let serialized = serde_json::to_vec(&message)?;
        let _ = self.destination.log(serialized);

        Ok(())
    }
}

#[allow(clippy::let_and_return)]
fn timestamp() -> f64 {
    let now = chrono::Utc::now();
    let milliseconds = (now.timestamp() as f64) + (now.timestamp_subsec_millis() as f64) / 1E3;
    milliseconds
}
