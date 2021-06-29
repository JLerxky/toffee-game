use protocol::{
    data::{control_data::ControlData, skill_data::SkillData},
    packet::Packet,
    route::GameRoute,
};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Receiver, Sender};

use std::str::FromStr;
use std::{net::SocketAddr, sync::Arc};

pub async fn net_server_start(net_tx: Sender<Packet>, engine_rx: Receiver<Packet>) {
    if let Ok(game_server_socket) = UdpSocket::bind(config::SERVER_ADDR).await {
        if let Ok(game_server_addr) = &game_server_socket.local_addr() {
            println!("网络服务器已启动: {:?}", game_server_addr);

            let r = Arc::new(game_server_socket);
            let s = r.clone();

            // let game_server_framed = UdpFramed::new(**r, BytesCodec::new());

            let game_server_future = start_listening(r, s.clone(), net_tx);

            let wait_for_send_future = wait_for_send(s, engine_rx);
            let clean_offline_user_future = clean_offline_user();

            tokio::spawn(clean_offline_user_future);
            tokio::spawn(wait_for_send_future);
            game_server_future.await;
        }
    }
}

pub async fn send(socket: Arc<UdpSocket>, packet: Vec<u8>, recv_addr: SocketAddr) {
    let _ = socket.send_to(&packet[..], recv_addr).await;
}

pub async fn clean_offline_user() {
    let clean_tick = 5000u128;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(clean_tick as u64));
    loop {
        interval.tick().await;
    }
}

pub async fn multicast(socket: Arc<UdpSocket>, group: u32, packet: Vec<u8>) {
    // mo ni
    let data = "";
    if data.len() > 0 {
        // println!("在线玩家IP: {}", &data);
        let uid_list: Vec<&str> = data.split(",").collect();
        for index in 0..uid_list.len() {
            let recv_addr = SocketAddr::from_str(uid_list[index]).unwrap();
            let socket = socket.clone();
            let packet = packet.clone();
            tokio::spawn(send(socket, packet, recv_addr));
            // senders.push(sender);
        }
    }
}

pub async fn wait_for_send(socket: Arc<UdpSocket>, mut engine_rx: Receiver<Packet>) {
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10));
    loop {
        // interval.tick().await;
        if let Some(packet) = engine_rx.recv().await {
            // println!("{:?}", packet);
            let socket = socket.clone();
            multicast(socket, 0, bincode::serialize(&packet).unwrap()).await;
            // let _ = tokio::join!(multicast(socket, 0, bincode::serialize(&packet).unwrap()));
        }
    }
}

pub async fn start_listening(
    socket: Arc<UdpSocket>,
    send_socket: Arc<UdpSocket>,
    net_tx: Sender<Packet>,
) {
    let mut buf = [0; config::PACKET_SIZE];
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_nanos(10));
    loop {
        // interval.tick().await;
        // println!("接收ing");
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            // println!("服务器收到数据: {}", &len);
            if let Ok(packet) = bincode::deserialize::<Packet>(&buf[..len]) {
                // println!("服务器收到数据: {:?}", &packet);
                // 转发事件
                match packet {
                    Packet::Heartbeat(heartbeat_route) => match heartbeat_route {
                        protocol::route::HeartbeatRoute::In => {}
                        protocol::route::HeartbeatRoute::Out => {}
                        protocol::route::HeartbeatRoute::Keep(_) => {
                            // 回ping
                            let _ =
                                tokio::spawn(send(send_socket.clone(), buf[..len].to_vec(), addr));
                            // TODO 添加玩家ip地址健康检查状态
                        }
                    },
                    Packet::Account(account_route) => match account_route {
                        protocol::route::AccountRoute::Login(account_data) => {
                            println!("{}登录事件: {:?}", &addr, &account_data);
                            // TODO 根据玩家ip注册或获取uid
                            let uid = account_data.uid;

                            // TODO 更新在线玩家表

                            // TODO 更新玩家组ip地址

                            // TODO 添加玩家ip地址健康检查状态

                            // TODO 发送生成玩家实体事件到引擎
                            let packet_login =
                                Packet::Account(protocol::route::AccountRoute::Login(
                                    protocol::data::account_data::AccountData {
                                        uid,
                                        group: account_data.group,
                                    },
                                ));
                            let _ = net_tx.try_send(packet_login.clone());
                            let _ = tokio::spawn(send(
                                send_socket.clone(),
                                bincode::serialize(&packet_login).unwrap(),
                                addr,
                            ));
                        }
                        protocol::route::AccountRoute::Logout(account_data) => {
                            println!("{}登出事件: {:?}", &addr, &account_data);
                            // TODO 根据玩家ip获取uid

                            // TODO 更新在线玩家表

                            // TODO 更新玩家组ip地址
                        }
                        protocol::route::AccountRoute::GetInfo(account_data) => {
                            // TODO 根据玩家ip获取uid
                            let uid = account_data.uid;

                            // 发送生成玩家实体事件到引擎
                            let packet_login =
                                Packet::Account(protocol::route::AccountRoute::Login(
                                    protocol::data::account_data::AccountData {
                                        uid,
                                        group: account_data.group,
                                    },
                                ));
                            let _ = tokio::spawn(send(
                                send_socket.clone(),
                                bincode::serialize(&packet_login).unwrap(),
                                addr,
                            ));
                        }
                    },
                    Packet::Game(game_route) => match game_route {
                        protocol::route::GameRoute::Control(control_data) => {
                            // println!("{}控制: {:?}", &addr, &control_data);
                            // TODO 根据玩家ip注册或获取uid
                            let uid = control_data.uid;

                            let packet_control = Packet::Game(GameRoute::Control(ControlData {
                                uid,
                                direction: control_data.direction,
                                action: control_data.action,
                            }));
                            if let Ok(_) = net_tx.try_send(packet_control) {
                                // println!("{}转递控制: {:?}", &addr, &control_data);
                            }
                        }
                        protocol::route::GameRoute::Tile(tile_state) => {}
                        GameRoute::Skill(skill_data) => {
                            let uid = skill_data.uid;
                            // TODO 根据玩家ip注册或获取uid
                            let packet_control = Packet::Game(GameRoute::Skill(SkillData {
                                uid,
                                direction: skill_data.direction,
                                skill_type: skill_data.skill_type,
                                texture: skill_data.texture,
                            }));
                            if let Ok(_) = net_tx.try_send(packet_control) {
                                // println!("{}转递控制: {:?}", &addr, &control_data);
                            }
                        }
                        GameRoute::TileMap(_tile_map_data) => {}
                        GameRoute::Update(_) => {}
                        GameRoute::Player(_) => {}
                        GameRoute::PlayerList(_) => {}
                    },
                    // _ => println!("{}收到事件未处理: {:?}", &addr, &packet),
                }
                // socket.send((Bytes::from("收到！"), addr)).await.unwrap();
            }
        }
    }
}
