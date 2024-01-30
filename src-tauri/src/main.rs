// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] extern crate rocket;

use rocket::http::Method;
use crate::master_server::MasterServer;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use crate::routes::{get_new_servers, trigger_fetch_servers};
use crate::settings::Settings;

mod game_server;
mod master_server;
mod regions;
mod routes;
mod settings;
mod player;

#[tokio::main]
async fn main() {
    let master_server = MasterServer::new(String::from("hl2master.steampowered.com:27011"));
    let mut settings = Settings::default();
    settings.load("settings.json").expect("Unable to load settings");
    tauri::Builder::default()
        .setup(move |_app| {
            let master_server_clone = master_server.clone();
            let cors = CorsOptions {
                allowed_origins: AllowedOrigins::all(),
                allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
                allowed_headers: AllowedHeaders::all(),
                ..Default::default()
            }
                .to_cors()
                .expect("CORS configuration error");

            tauri::async_runtime::spawn(
                async move {
                    master_server_clone.fetch_servers(settings.region, settings.filter).await;
                    rocket::build()
                        .attach(cors)
                        .manage(master_server_clone)
                        .mount("/", rocket::routes![get_new_servers, trigger_fetch_servers])
                        .launch()
                        .await
                        .expect("Rocket failed to launch");
                }
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

