use crate::game_server::GameServer;
use crate::master_server::MasterServer;
use std::time::Instant;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/new-servers")]
pub fn get_new_servers(state: &State<MasterServer>) -> Json<Vec<GameServer>> {
    let mut last_fetched_guard = state.last_fetched.lock().unwrap();
    let new_servers: Vec<GameServer> = {
        let game_servers_guard = state.game_servers.lock().unwrap();
        game_servers_guard
            .iter()
            .filter(|(_addr, server)| server.last_updated > *last_fetched_guard)
            .map(|(_, server)| server.clone())
            .collect()
    };
    *last_fetched_guard = Instant::now();
    Json(new_servers)
}

#[post("/fetch-servers")]
pub async fn trigger_fetch_servers(state: &State<MasterServer>) -> Status {
    state.inner().fetch_servers(0x02, String::from("\\appid\\346110")).await;
    Status::Ok
}
