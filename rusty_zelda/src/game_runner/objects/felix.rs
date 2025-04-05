use super::item;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
const TILE_SIZE: u32 = 32;
const ROOM_WIDTH: u32 = 512;
const ROOM_HEIGHT: u32 = 352;
const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 11;
pub struct Felix{ //Name of character (Fe)lix RedOx haha!
    inventory: Vec<item::Item>,
    health_bar: u8, //2*(heart count)
    rupee_balance: i32,
    hand: (Option<item::Item>, Option<item::Item>),
    location: (u32, u32),
    size: u32,
    speed: f32,
}
impl Felix{
    pub fn new(size: u32) -> Self {
        Self{inventory: vec!(), health_bar: 6, rupee_balance: 0, hand: (None, None), location: (0, 0), size, speed: 0.0}
    }
    pub fn move_felix(&mut self, x: u32, y: u32, pos: bool){ //Keep in mind this function doesn't teleport felix but add's given position
        match pos{
            true => {self.location.0 += x; self.location.1 += y;},
            false => {self.location.0 -= x; self.location.1 -= y;},
        }
    }
    pub fn use_hand_a(&mut self){
        if let Some(item) = self.hand.0.as_mut(){
            item.use_item(self.location, &mut self.rupee_balance);
            if item.is_disposable(){
                item.count_remove();
                if item.count() == &Some(0) {
                    self.hand.0 = None;
                }
            }
        }
    }
    pub fn use_hand_b(&mut self){
        if let Some(item) = self.hand.1.as_mut(){
            item.use_item(self.location, &mut self.rupee_balance);
            if item.is_disposable(){
                item.count_remove();
                if item.count() == &Some(0) {
                    self.hand.1 = None;
                }
            }
        }
    }
    fn rect(&self) -> Rect{
        Rect::new(self.location.0 as i32, self.location.1 as i32, self.size, self.size)
    }
    fn in_collision(x: f32, y:f32, size: f32, word_dungeon: Vec<Vec<u8>>, enem_corner: [(f32, f32); 4]) -> bool {
        let corners: [(f32, f32); 4]  = [
            (x, y),
            (x + size - 1.0, y),
            (x, y + size - 1.0),
            (x + size - 1.0, y + size - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            if word_dungeon[tile_y][tile_x] % 2 == 1 {return true};
            if (cx <= enem_corner[1].0 && cx <= enem_corner[3].0) 
                && 
                (cy <= enem_corner[1].1 && cy <= enem_corner[2].1){
                return true;
            }
        }
        false
    }
}