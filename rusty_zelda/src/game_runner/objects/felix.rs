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
    location: (f32, f32),
    size: u32,
    speed: f32,
}
impl Felix{
    pub fn new(size: u32) -> Self {
        Self{inventory: vec!(), health_bar: 6, rupee_balance: 0, hand: (None, None), location: (0.0, 0.0), size, speed: 0.0}
    }
    pub fn move_felix(&mut self, keys: sdl2::keyboard::KeyboardState<'_>, world_dungeon: &Vec<Vec<u8>>, enem_corner: &Vec<[(f32, f32); 4]>){ //Keep in mind this function doesn't teleport felix but add's given position
        let (mut dx, mut dy): (f32, f32) = (0.0, 0.0);
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Left){
            dx -= self.speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Right){
            dx += self.speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Up){
            dy -= self.speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Down){
            dy += self.speed;
        }
        let (new_x, new_y) = (self.location.0 as f32 + dx, self.location.1 as f32 + dy);

        if !Felix::in_collision(new_x, self.location.1, self.size as f32, world_dungeon, enem_corner){
            self.location.0 = new_x;
        }
        if !Felix::in_collision(self.location.0, new_y, self.size as f32, world_dungeon, enem_corner){
            self.location.1 = new_y;
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
    fn in_collision(x: f32, y:f32, size: f32, world_dungeon: &Vec<Vec<u8>>, enem_corner: &Vec<[(f32, f32); 4]>) -> bool {
        let corners: [(f32, f32); 4]  = [
            (x, y),
            (x + size - 1.0, y),
            (x, y + size - 1.0),
            (x + size - 1.0, y + size - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            if world_dungeon[tile_y][tile_x] % 2 == 1 {return true};
            for enemy in enem_corner{
                if (cx <= enemy[1].0 && cx <= enemy[3].0) 
                && 
                (cy <= enemy[1].1 && cy <= enemy[2].1){
                return true;
                }
            }
        }
        false
    }
}