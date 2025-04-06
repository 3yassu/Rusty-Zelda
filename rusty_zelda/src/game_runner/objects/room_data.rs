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
    pub shop_keeper: npc::Shopkeeper //shopkeeper handles items, etc
}
impl ShopRoomData{
    pub fn new(player_spawn: (f32, f32), dungeon: Vec<Vec<u8>>, shop_keeper: npc::Shopkeeper) -> Self{
        Self{player_spawn, dungeon, shop_keeper}
    }
    pub fn access_spawn(&mut self) -> &mut (f32, f32){&mut self.player_spawn}
}
#[derive(Debug)]
pub struct HostileRoomData{
    pub player_spawn: (f32, f32),
    pub dungeon: Vec<Vec<u8>>,
    pub enemies: Vec<npc::Enemy>,
    pub items: Vec<item::Item>
}
impl HostileRoomData{
    pub fn new(player_spawn: (f32, f32), dungeon: Vec<Vec<u8>>, enemies: Vec<npc::Enemy>, items: Vec<item::Item>) -> Self{
        Self{player_spawn, dungeon, enemies, items}
    }
    pub fn access_spawn(&mut self) -> &mut (f32, f32){&mut self.player_spawn}
    pub fn get_enemy(&mut self) -> &mut Vec<npc::Enemy>{&mut self.enemies}
}