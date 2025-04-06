use super::item;
use sdl2::rect::Rect;
//two types of npc: shopkeeper and enemy

#[derive(Debug)]
pub struct Shopkeeper {
    items: Vec<item::Item>,
    //text_gen
    //item_text_gen?
    location: (u32, u32),
    collision: bool
}

impl Shopkeeper {
    pub fn new(items: Vec<item::Item>, location: (u32, u32), collision: bool) -> Self {
        Self{items, location, collision}
    }
    //shopkeeper and items lose visibility after felix has get_item animation
}

#[derive(Debug)]
pub struct Enemy {
    item_on_kill: item::Item,
    id: u32,
    hp: u32,
    collision: bool,
    ignore_room_collision: bool,
    location: (u32, u32),
    size: u32
    //animations
}

impl Enemy {
    pub fn new(item_on_kill: item::Item, id: u32, hp: u32, collision: bool, ignore_room_collision: bool, location: (u32, u32), size: u32) -> Self {
        Self{item_on_kill, id, hp, collision, ignore_room_collision, location, size}
    }
    //enemy movement
    pub fn move_enemy(&mut self, x: u32, y: u32, pos: bool){
        match pos{
            true => {self.location.0 += x; self.location.1 += y;},
            false => {self.location.0 -= x; self.location.1 -= y;},
        }
    }
    //enemy ai -- attacks need to be based on enemy id/type.
    pub fn keese(location: (u32, u32)) -> Self{ //quick debug functions
        Self{item_on_kill: item::Item::new(0, 0, None, false, (None, None)), id: 30, collision: true, hp: 1, ignore_room_collision: true, location, size: 8}
    }
    pub fn stalfos(location: (u32, u32)) -> Self{ //quick debug functions
        Self{item_on_kill: item::Item::new(0, 0, None, false, (None, None)), id: 40, collision: true, hp: 2, ignore_room_collision: false, location, size: 16}
    }
    pub fn rect(&self) -> Rect{
        Rect::new(self.location.0 as i32, self.location.1 as i32, self.size, self.size)
    }
}
