mod message;
mod level;
mod udp;
mod chunked;

extern crate slog;
extern crate chrono;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate flate2;

use std::io;
use slog::{
    Drain,
    Record,
    OwnedKVList,
    KV,
    Key,
};
use flate2::{write::GzEncoder, Compression};
use std::io::prelude::*;

use message::Message;
use udp::UdpDestination;
use chunked::ChunkSize;

static VERSION: &'static str = "1.1";

pub struct Gelf {
    source          : String,
    destination     : UdpDestination,
}

impl Gelf {
    pub fn new(source: &str, destination: &str) -> Result<Self, io::Error> {
        let destination = UdpDestination::new(destination, ChunkSize::LAN)?;

        Ok(Gelf{
            source: source.to_owned(),
            destination,
        })
    }
}

pub struct KeyValueList(pub Vec<(&'static str, String)>);

impl slog::Serializer for KeyValueList {
    fn emit_arguments(&mut self, key: Key, val: &std::fmt::Arguments) -> slog::Result {
        self.0.push((key as &'static str, format!("{}", val)));
        Ok(())
    }
}

impl Drain for Gelf {
    type Ok = ();
    type Err = io::Error;

    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> Result<Self::Ok, Self::Err>
    {
        let mut additional = KeyValueList(Vec::with_capacity(16));
        record.kv().serialize(record, &mut additional)?;
        values.serialize(record, &mut additional)?;

        let message = Message {
            version         : VERSION,
            host            : &self.source,
            short_message   : record.msg().to_string(),
            full_message    : None,
            timestamp       : Some(timestamp()),
            level           : Some(record.level().into()),
            module          : Some(record.location().module),
            file            : Some(record.location().file),
            line            : Some(record.location().line),
            column          : None,
            additional      : additional.0,
        };

        let serialized = serde_json::to_vec(&message)?;

        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(&serialized)?;
        let compressed = e.finish()?;
        let _ = self.destination.log(compressed);

        Ok(())
    }
}

fn timestamp() -> f64 {
    let now = chrono::Utc::now();
    let milliseconds = (now.timestamp() as f64) + (now.timestamp_subsec_millis() as f64) / 1E3;
    milliseconds
}