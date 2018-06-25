mod message;
mod level;

extern crate slog;
extern crate log;
extern crate hostname;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use slog::{
    Drain,
    Record,
    OwnedKVList,
    KV,
    Key,
};

use message::Message;

static VERSION: &'static str = "1.1";

pub struct Gelf {
    hostname: String,
}

impl Gelf {
    pub fn new() -> Self {
        let hostname = hostname::get_hostname().unwrap_or("hostname".to_string());

        Gelf {
            hostname,
        }
    }

    pub fn new_with_hostname(hostname: &str) -> Self {
        Gelf {
            hostname: hostname.to_owned(),
        }
    }
}

pub struct KeyValueList(pub Vec<(String, String)>);

impl slog::Serializer for KeyValueList {
    fn emit_arguments(&mut self, key: Key, val: &std::fmt::Arguments) -> slog::Result {
        self.0.push((format!("_{}", key), format!("{}", val)));
        Ok(())
    }
}

impl Drain for Gelf {
    type Ok = ();
    type Err = std::io::Error;

    fn log(
        &self,
        record: &Record,
        _values: &OwnedKVList,
    ) -> Result<Self::Ok, Self::Err>
    {
        let now = chrono::Utc::now();
        let milliseconds = (now.timestamp() as f64) + (now.timestamp_subsec_millis() as f64) / 1E3;

        let mut logmap = KeyValueList(Vec::new());
        record.kv().serialize(record, &mut logmap)?;

        let message = Message {
            version         : VERSION,
            host            : self.hostname.clone(),
            short_message   : record.msg().to_string(),
            full_message    : None,
            timestamp       : Some(milliseconds),
            level           : Some(record.level().into()),
            module          : Some(record.location().module.to_owned()),
            file            : Some(record.location().file.to_owned()),
            line            : Some(record.location().line),
            column          : None,
            additional      : logmap.0,
        };

        println!("{}", serde_json::to_string(&message).unwrap());

        Ok(())
    }
}