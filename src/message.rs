use serde::ser::{Serialize, Serializer, SerializeMap};

use level::Level;

pub struct Message<'a> {
    pub version             : &'static str,
    pub host                : &'a str,
    pub short_message       : String,
    pub full_message        : Option<String>,
    pub timestamp           : Option<f64>,
    pub level               : Option<Level>,
    pub module              : Option<&'static str>,
    pub file                : Option<&'static str>,
    pub line                : Option<u32>,
    pub column              : Option<u32>,
    pub additional          : Vec<(&'static str, String)>,
}

impl<'a> Serialize for Message<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(10 + self.additional.len()))?;

        map.serialize_key("version")?;
        map.serialize_value(self.version)?;

        map.serialize_key("host")?;
        map.serialize_value(&self.host)?;

        map.serialize_key("short_message")?;
        map.serialize_value(&self.short_message)?;

        if let Some(ref timestamp) = self.timestamp {
            map.serialize_key("timestamp")?;
            map.serialize_value(timestamp)?;
        }

        if let Some(ref level) = self.level {
            map.serialize_key("level")?;
            map.serialize_value(level)?;
        }

        if let Some(ref module) = self.module {
            map.serialize_key("_module")?;
            map.serialize_value(module)?;
        }

        if let Some(ref file) = self.file {
            map.serialize_key("_file")?;
            map.serialize_value(file)?;
        }

        if let Some(ref line) = self.line {
            map.serialize_key("_line")?;
            map.serialize_value(line)?;
        }

        if let Some(ref column) = self.column {
            map.serialize_key("_column")?;
            map.serialize_value(column)?;
        }

        for (key, value) in self.additional.iter().rev() {
            map.serialize_key(&format!("_{}", key))?;
            map.serialize_value(value)?;
        }

        map.end()
    }
}