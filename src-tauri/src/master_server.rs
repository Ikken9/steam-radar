use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::net::UdpSocket;
use a2s;
use a2s::{A2SClient};
use crate::game_server::GameServer;
use crate::player::Player;

#[derive(Clone)]
pub struct MasterServer {
    address: String,
    pub game_servers: Arc<Mutex<HashMap<SocketAddrV4, GameServer>>>,
    pub last_fetched: Arc<Mutex<Instant>>,
}

pub struct MasterServerQueryBuffer {
    message_type: u8,
    region: u8,
    pub addr: SocketAddrV4,
    filter: String
}

#[derive(Debug)]
pub struct SendRecvError {
    message: String
}

pub enum MasterServerError {
    SendRecvError,

}

impl MasterServer {
    pub fn new(address: String) -> MasterServer {
        MasterServer {
            address,
            game_servers: Arc::new(Mutex::new(HashMap::new())),
            last_fetched: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn fetch_servers(&self, region: u8, filter: String) {
        let local_socket_addr = SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 0);
        let mut ms_buffer = MasterServerQueryBuffer::new(0x31, region, local_socket_addr, filter);
        let udp_socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("Unable to create a UDP Socket");

        let eod_addr = SocketAddrV4::new(Ipv4Addr::new(0x00, 0x00, 0x00, 0x00), 0);

        loop {
            if let Ok(addresses) = send_recv(&udp_socket, &ms_buffer.to_vec(), &self.address).await {
                let last_buffer_addr = buffer_to_socket_addr(&addresses[addresses.len() - 6..]);
                save_servers(&addresses, &self.game_servers).await;
                ms_buffer.addr = last_buffer_addr;
                if last_buffer_addr == eod_addr {
                    break;
                }
            } else {
                eprintln!("Error during send/receive");
                break;
            }
        }
    }
}

impl MasterServerQueryBuffer {
    pub fn new(message_type: u8, region: u8, addr: SocketAddrV4, filter: String) -> MasterServerQueryBuffer {
        MasterServerQueryBuffer {
            message_type, region, addr, filter
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let octets = self.addr.ip().octets();
        let port = self.addr.port().to_be_bytes();
        let mut buffer = vec![
            self.message_type,
            self.region,
            octets[0],
            octets[1],
            octets[2],
            octets[3],
            port[0],
            port[1],
            0x00
        ];
        buffer.extend(self.filter.bytes());
        buffer.push(0x00);
        buffer
    }
}

impl Display for SendRecvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<String> for SendRecvError {
    fn from(value: String) -> Self {
        SendRecvError { message: value }
    }
}

impl Error for SendRecvError {}

async fn send_recv(udp_socket: &UdpSocket, buffer: &[u8], target: &str) -> Result<Vec<u8>, SendRecvError> {
    udp_socket.send_to(buffer, target)
        .await
        .map_err(|err| SendRecvError::from(format!("Unable to send UDP packet: {}", err)))?;

    let mut response_buffer = vec![0u8; 1026];
    let size = udp_socket.recv(&mut response_buffer)
        .await
        .map_err(|err| SendRecvError::from(format!("Unable to receive UDP response: {}", err)))?;

    response_buffer.truncate(size);
    Ok(response_buffer)
}

async fn save_servers(buffer: &[u8], game_servers: &Arc<Mutex<HashMap<SocketAddrV4, GameServer>>>) {
    for addr in buffer.chunks(6) {
        let socket_addr = buffer_to_socket_addr(addr);
        match get_game_server(socket_addr).await {
            Ok(game_server) => {
                if let Ok(mut guard) = game_servers.lock().map_err(|e| e.to_string()) {
                    guard.insert(socket_addr, game_server);
                }
            },
            Err(e) => {
                eprintln!("Error while trying to save server info: {}", e)
            }
        }
    }
}

async fn get_game_server(server_socket_addr: SocketAddrV4) -> Result<GameServer, Box<dyn Error>> {
    let mut gs: GameServer = GameServer::new(String::from("n/a"), String::from("n/a"), 0, 0, vec![]);
    match A2SClient::new() {
        Ok(client) => {
            match client.info(server_socket_addr) {
                Ok(info) => {
                    gs.name = info.name;
                    gs.map = info.map;
                    gs.player_count = info.players;
                    gs.max_players = info.max_players;
                }
                Err(e) => {
                    eprintln!("Failed to retrieve server info: {:?}", e)
                }
            }
            match client.players(server_socket_addr) {
                Ok(players) => {
                    for p in players {
                        let player = Player::new(p.name, p.score, p.duration);
                        gs.player_list.push(player)
                    }
                }
                Err(e) => {
                    eprintln!("Failed to retrieve players: {:?}", e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to server {}", e)
        }
    }
    Ok(gs)
}

fn buffer_to_socket_addr(addr_slice: &[u8]) -> SocketAddrV4 {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(
        addr_slice[0],
        addr_slice[1],
        addr_slice[2],
        addr_slice[3]), ((addr_slice[4] as u16) << 8) | addr_slice[5] as u16);
    socket_addr
}
