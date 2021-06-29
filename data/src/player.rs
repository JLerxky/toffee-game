use std::error::Error;

use protocol::data::player_data::PlayerData;

#[derive(Debug)]
pub struct GameData {
    pub table: String,
    pub key: String,
    pub data: Option<String>,
}

impl GameData {
    pub fn player_online(data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: "online".to_string(),
            data,
        }
    }
    pub fn player_group_addr(group: u32, data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: format!("group_addr_{}", group),
            data,
        }
    }
    pub fn player_addr_health(addr: String, data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: format!("addr_health_{}", addr),
            data,
        }
    }
    pub fn player_addr_uid(addr: String, data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: format!("addr_uid_{}", addr),
            data,
        }
    }
    pub fn player_queue_uid(data: Option<String>) -> Self {
        GameData {
            table: "player".to_string(),
            key: "queue_uid".to_string(),
            data,
        }
    }
}

pub fn save(data: GameData) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn find(key: GameData) -> Result<String, Box<dyn Error>> {
    todo!()
}

pub fn next_u64(key: GameData) -> Result<u64, Box<dyn Error>> {
    todo!()
}

pub fn save_player(player: PlayerData) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn find_player(uid: u32) -> Result<PlayerData, Box<dyn Error>> {
    todo!()
}

pub fn next_entity_id(entity_type: u8) -> Result<u64, Box<dyn Error>> {
    todo!()
}
