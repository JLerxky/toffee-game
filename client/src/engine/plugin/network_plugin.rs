use std::{
    io,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use protocol::{
    data::{player_data::PlayerData, update_data::EntityType},
    packet::Packet,
    route::{AccountRoute, GameRoute, HeartbeatRoute},
};
use tokio::net::UdpSocket;

// 当前玩家
pub static mut PLAYER: PlayerData = PlayerData {
    uid: 0,
    hp: 0,
    mp: 0,
    max_hp: 100,
    max_mp: 100,
};

pub struct NetWorkState {
    pub packet_queue: Arc<Mutex<Vec<Packet>>>,
    pub to_be_sent_queue: Arc<Mutex<Vec<Packet>>>,
}

pub struct SynEntity {
    pub id: u64,
    pub entity_type: EntityType,
    pub health: u64,
    pub animate_type: u8,
    pub animate_index: usize,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let packet_queue: Vec<Packet> = Vec::new();
        let packet_queue = Arc::new(Mutex::new(packet_queue));
        let packet_queue_c = packet_queue.clone();

        let to_be_sent_queue: Vec<Packet> = Vec::new();
        let to_be_sent_queue = Arc::new(Mutex::new(to_be_sent_queue));
        let to_be_sent_queue_c = to_be_sent_queue.clone();

        let net_client_runtime = tokio::runtime::Runtime::new().unwrap();
        net_client_runtime.spawn(net_client_start(packet_queue_c, to_be_sent_queue_c));

        app.insert_resource(NetWorkState {
            packet_queue,
            to_be_sent_queue,
        })
        .add_system(net_handler_system.system());
    }
}

fn net_handler_system(
    net_state: ResMut<NetWorkState>,
    // mut hb_event_writer: EventWriter<HeartBeatEvent>,
    // mut sync_event_writer: EventWriter<SyncEvent>,
    // mut player_event_writer: EventWriter<PlayerUpdateEvent>,
) {
    if let Ok(mut packet_queue) = net_state.packet_queue.lock() {
        // println!("packet_queue: {}", packet_queue.len());
        for _ in 0..10 {
            if packet_queue.is_empty() {
                return;
            }
            let packet = packet_queue[0].clone();
            packet_queue.remove(0);

            match packet {
                Packet::Heartbeat(heartbeat_route) => match heartbeat_route {
                    HeartbeatRoute::In => {}
                    HeartbeatRoute::Out => {}
                    protocol::route::HeartbeatRoute::Keep(time) => {
                        todo!()
                    }
                },
                Packet::Account(account_route) => match account_route {
                    AccountRoute::Login(login_data) => unsafe {
                        PLAYER.uid = login_data.uid;
                    },
                    AccountRoute::Logout(_) => {}
                    AccountRoute::GetInfo(account_data) => unsafe {
                        PLAYER.uid = account_data.uid;
                    },
                },
                Packet::Game(game_route) => match game_route {
                    GameRoute::Update(update_data) => {
                        todo!()
                    }
                    GameRoute::Control(_control_data) => {}
                    GameRoute::TileMap(tile_map_data) => {}
                    GameRoute::Tile(tile_state) => {}
                    GameRoute::Player(_) => {}
                    GameRoute::PlayerList(player_list_data) => {
                        todo!()
                    }
                    GameRoute::Skill(_) => {}
                },
            }
        }
    }
}

async fn net_client_start(
    packet_queue: Arc<Mutex<Vec<Packet>>>,
    to_be_sent_queue: Arc<Mutex<Vec<Packet>>>,
) -> io::Result<()> {
    // 连接服务器
    println!("客户端网络连接ing...");
    let sock = UdpSocket::bind(config::CLIENT_ADDR).await?;
    let remote_addr = config::SERVER_ADDR;
    sock.connect(remote_addr).await?;
    println!("客户端网络连接成功: {:?}", sock.local_addr());
    let r = Arc::new(sock);
    let s = r.clone();

    // TODO 登录服务器

    // 维持心跳
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(2f64));
        loop {
            interval.tick().await;
            // TODO 发送心跳

            // 未登录则重新登录
            unsafe {
                if PLAYER.uid == 0 {
                    // TODO 登录服务器
                }
            }
        }
    });

    let mut buf = [0; config::PACKET_SIZE];
    loop {
        // println!("接收ing");
        if let Ok(len) = r.recv(&mut buf).await {
            // println!("接收来自服务器的 {:?} bytes", len);
            let packet = bincode::deserialize(&buf[..len]);
            // 转发事件
            if let Ok(packet) = packet {
                if let Ok(mut packet_queue) = packet_queue.lock() {
                    if packet_queue.len() > 512 {
                        packet_queue.remove(0);
                    }
                    packet_queue.push(packet);
                }
            }
        }

        if let Ok(mut to_be_sent_queue) = to_be_sent_queue.lock() {
            let to_be_sent_queue_c = to_be_sent_queue.clone();
            to_be_sent_queue.clear();
            for to_be_sent_packet in to_be_sent_queue_c.iter() {
                let s = r.clone();
                let to_be_sent_packet = to_be_sent_packet.clone();
                tokio::spawn(async move {
                    s.send(&bincode::serialize(&to_be_sent_packet).unwrap()[0..])
                        .await
                        .unwrap();
                });
            }
        }
    }
}
