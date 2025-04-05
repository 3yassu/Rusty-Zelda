//Nothing for now :)
use super::item;
use super::npc;
#[derive(Debug)]
pub enum RoomData{
    Shop(ShopRoomData),
    Hostile(HostileRoomData)
}
#[derive(Debug)]
pub struct ShopRoomData{//room item info & image data
    dimensions: (usize, usize),
    player_spawn: (f32, f32),
    dungeon: Vec<Vec<u8>>,
    shopkeeper: npc::Shopkeeper //shopkeeper handles items, etc
}

#[derive(Debug)]
pub struct HostileRoomData{
    dimensions: (usize, usize),
    player_spawn: (f32, f32),
    dungeon: Vec<Vec<u8>>,
    enemies: Vec<npc::Enemy>,
    items: Vec<item::Item>
}