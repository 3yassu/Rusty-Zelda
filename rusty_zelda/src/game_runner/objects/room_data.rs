//Nothing for now :)
use super::item;
use super::npc;
pub enum RoomData{
    Shop(ShopRoomData),
    Hostile(HostileRoomData)
}
#[derive(Debug)]
pub struct ShopRoomData{//room item info & image data
    dimensions: (usize, usize),
    player_spawn: (f32, f32),
    dungeon: [[u8; dimensions.0]; dimensions.1],
    shopkeeper: npc::Shopkeeper //shopkeeper handles items, etc
}

#[derive(Debug)]
pub struct HostileRoomData{
    dimensions: (usize, usize),
    player_spawn: (f32, f32),
    dungeon: [[u8; dimensions.0]; dimensions.1],
    enemies: Vec<npc::Enemy>,
    items: Vec<item::Item>
}