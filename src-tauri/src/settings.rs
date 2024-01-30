use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;

pub struct Settings {
    pub filter: String,
    pub region: u8,
}

impl Settings {
    pub fn new(filter: String, region: u8) -> Settings {
        Settings {
            filter,
            region
        }
    }

    pub fn default() -> Settings {
        Settings {
            filter: "\\appid\\730\\".to_string(),
            region: 0x02
        }
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string(self)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        let file_path = Path::new(path);
        if let Ok(file) = File::open(file_path) {
            match serde_json::from_reader::<File, Settings>(file) {
                Ok(settings) => {
                    self.region = settings.region;
                    self.filter = settings.filter;
                }
                Err(e) => {
                    eprint!("Unable to parse settings.json: {}", e)
                }
            }
        } else {
            self.region = 0x02;
            self.filter = String::from("\\appid\\730\0");
        }

        Ok(())
    }
}

impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Settings", 2)?;
        state.serialize_field("region", &self.region)?;
        state.serialize_field("filter", &self.filter)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Region,
            Filter
        }

        struct SettingsVisitor;

        impl<'de> Visitor<'de> for SettingsVisitor {
            type Value = Settings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Settings")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Settings, V::Error> where V: SeqAccess<'de> {
                let region = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let filter = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Settings::new(region, filter))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Settings, V::Error> where V: MapAccess<'de> {
                let mut region = None;
                let mut filter = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Region => {
                            if region.is_some() {
                                return Err(de::Error::duplicate_field("region"));
                            }
                            region = Some(map.next_value()?);
                        }
                        Field::Filter => {
                            if filter.is_some() {
                                return Err(de::Error::duplicate_field("filter"));
                            }
                            filter = Some(map.next_value()?);
                        }
                    }
                }
                let region = region.ok_or_else(|| de::Error::missing_field("region"))?;
                let filter = filter.ok_or_else(|| de::Error::missing_field("filter"))?;
                Ok(Settings::new(region, filter))
            }
        }
        const FIELDS: &'static [&'static str] = &["region", "filter"];
        deserializer.deserialize_struct("Settings", FIELDS, SettingsVisitor)
    }
}
