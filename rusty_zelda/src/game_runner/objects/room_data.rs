use super::item;
use super::npc;
#[derive(Debug)]
pub enum RoomData{
    Shop(ShopRoomData),
    Hostile(HostileRoomData)
}
#[derive(Debug)]
pub struct ShopRoomData{//room item info & image data
    pub dungeon: Vec<Vec<u8>>,
    pub shop_keeper: npc::Shopkeeper //shopkeeper handles items, etc
}
impl ShopRoomData{
    pub fn new(dungeon: Vec<Vec<u8>>, shop_keeper: npc::Shopkeeper) -> Self{
        Self{dungeon, shop_keeper}
    }
}
#[derive(Debug)]
pub struct HostileRoomData{
    pub dungeon: Vec<Vec<u8>>,
    pub enemies: Vec<npc::Enemy>,
    pub items: Vec<item::Item>
}
impl HostileRoomData{
    pub fn new(dungeon: Vec<Vec<u8>>, enemies: Vec<npc::Enemy>, items: Vec<item::Item>) -> Self{
        Self{dungeon, enemies, items}
    }
    pub fn get_enemy(&mut self) -> &mut Vec<npc::Enemy>{&mut self.enemies}
    pub fn get_enemy_col_mut(&mut self) -> Vec<[(f32, f32); 4]>{
        let mut ret_vec: Vec<[(f32, f32); 4]> = vec!();
        for i in &self.enemies{
            ret_vec.push(i.get_col());
        }
        ret_vec
    }
    pub fn get_enemy_col(&self) -> Vec<[(f32, f32); 4]>{
        let mut ret_vec: Vec<[(f32, f32); 4]> = vec!();
        for i in &self.enemies{
            ret_vec.push(i.get_col());
        }
        ret_vec
    }
    pub fn get_enemy_ref(&mut self) -> &Vec<npc::Enemy>{&self.enemies}
}