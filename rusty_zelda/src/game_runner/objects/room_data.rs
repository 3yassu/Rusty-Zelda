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
    pub player_spawn: (f32, f32),
    pub dungeon: Vec<Vec<u8>>,
    pub shopkeeper: npc::Shopkeeper //shopkeeper handles items, etc
}

#[derive(Debug)]
pub struct HostileRoomData{
    pub player_spawn: (f32, f32),
    pub dungeon: Vec<Vec<u8>>,
    pub enemies: Vec<npc::Enemy>,
    pub items: Vec<item::Item>
}