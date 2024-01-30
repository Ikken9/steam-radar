use std::fmt::{Display, Formatter};
use std::time::Instant;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::player::Player;

#[derive(Clone)]
pub struct GameServer {
    pub name: String,
    pub map: String,
    pub player_count: u8,
    pub max_players: u8,
    pub last_updated: Instant,
    pub player_list: Vec<Player>
}

impl GameServer {
    pub fn new(name: String, map: String, player_count: u8, max_players: u8, player_list: Vec<Player>) -> GameServer {
        GameServer {
            name,
            map,
            player_count,
            max_players,
            player_list,
            last_updated: Instant::now(),
        }
    }

    // Method to update the GameServer and its last_updated field
    pub fn update(&mut self, /* new data parameters */) {
        // Update the GameServer fields with new data

        // Update the last_updated field
        self.last_updated = Instant::now();
    }
}

impl Display for GameServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}/{} {:?}", self.name, self.map, self.player_count, self.max_players, self.player_list)
    }
}

impl Serialize for GameServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("GameServer", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("map", &self.map)?;
        state.serialize_field("player_count", &self.player_count)?;
        state.serialize_field("max_players", &self.max_players)?;
        state.serialize_field("player_list", &self.player_list)?;
        state.end()
    }
}

