use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    score: i32,
    duration: f32
}

impl Player {
    pub fn new(name: String, score: i32, duration: f32) -> Player {
        Player {
            name,
            score,
            duration
        }
    }
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Player", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("score", &self.score)?;
        state.serialize_field("duration", &self.duration)?;
        state.end()
    }
}