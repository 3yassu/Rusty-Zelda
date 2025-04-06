use super::item;
use sdl2::rect::Rect;
use rand::Rng;
const TILE_SIZE: u32 = 32;

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
    location: (f32, f32),
    size: u32,
    speed: f32,
    delta_pos: (f32, f32),
    //animations
}

impl Enemy {
    pub fn new(item_on_kill: item::Item, id: u32, hp: u32, collision: bool, ignore_room_collision: bool, location: (f32, f32), size: u32, speed: f32, delta_pos: (f32, f32)) -> Self {
        Self{item_on_kill, id, hp, collision, ignore_room_collision, location, size, speed, delta_pos}
    }
    //enemy movement
    pub fn move_enemy(&mut self, world_dungeon: &Vec<Vec<u8>>, can_move: bool, keep_going: bool){
            let possible_x: f32 = rand::rng().random_range(0..=2) as f32;
            let possible_y: f32 = rand::rng().random_range(0..=2) as f32;
        match self.id{
            40 => {self.move_stalfos(possible_x, possible_y, world_dungeon, can_move)},
            30 => {self.move_keese(possible_x, possible_y, world_dungeon, can_move, keep_going)},
            _ => ()
        }
    }
    fn move_keese(&mut self, x: f32, y: f32, world_dungeon: &Vec<Vec<u8>>, can_move: bool, keep_going: bool){
        if !can_move {return;}
        if !keep_going{
            self.delta_pos = (0.0, 0.0);
            match x {
                2.0 => self.delta_pos.0 += self.speed,
                1.0 => self.delta_pos.0 -= self.speed,
                _ => ()
            }
            match y {
                2.0 => self.delta_pos.1 += self.speed,
                1.0 => self.delta_pos.1 -= self.speed,
                _ => ()       
            }
        }
        let (new_x, new_y) = (self.location.0 as f32 + self.delta_pos.0, self.location.1 as f32 + self.delta_pos.1);

        if !Enemy::enem_collision(new_x, self.location.1, self.size as f32, world_dungeon){
            self.location.0 = new_x;
        }
        if !Enemy::enem_collision(self.location.0, new_y, self.size as f32, world_dungeon){
            self.location.1 = new_y;
        }
    }
    fn move_stalfos(&mut self, x: f32, y: f32, world_dungeon: &Vec<Vec<u8>>, can_move: bool){
        if !can_move {return;}
        let (mut dx, mut dy): (f32, f32) = (0.0, 0.0);
        match x {
            2.0 => dx += self.speed,
            1.0 => dx -= self.speed,
            _ => ()
        }
        match y {
            2.0 => dy += self.speed,
            1.0 => dy -= self.speed,
            _ => ()            
        }
        let (new_x, new_y) = (self.location.0 as f32 + dx, self.location.1 as f32 + dy);

        if !Enemy::enem_collision(new_x, self.location.1, self.size as f32, world_dungeon){
            self.location.0 = new_x;
        }
        if !Enemy::enem_collision(self.location.0, new_y, self.size as f32, world_dungeon){
            self.location.1 = new_y;
        }
    }
    //enemy ai -- attacks need to be based on enemy id/type.
    pub fn keese(location: (f32, f32)) -> Self{ //quick debug functions
        Self{item_on_kill: item::Item::new(100, 0, None, false, (None, None), 0, 0.0), id: 30, collision: true, hp: 1, ignore_room_collision: true, location, size: 8, speed: 2.0, delta_pos: (0.0, 0.0)}
    }
    pub fn stalfos(location: (f32, f32)) -> Self{ //quick debug functions
        Self{item_on_kill: item::Item::new(100, 0, None, false, (None, None), 0, 0.0), id: 40, collision: true, hp: 2, ignore_room_collision: false, location, size: 16, speed: 4.0, delta_pos: (0.0, 0.0)}
    }
    pub fn rect(&self) -> Rect{
        Rect::new(self.location.0 as i32, self.location.1 as i32, self.size, self.size)
    }
    pub fn enem_collision(x: f32, y:f32, size: f32, world_dungeon: &Vec<Vec<u8>>) -> bool{
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
        }
        false
    }
    pub fn get_col(&self) -> [(f32, f32); 4]{
        [self.location, 
        (self.location.0 + (self.size as f32) - 1.0, self.location.1), 
        (self.location.0, self.location.1 + (self.size as f32) - 1.0), 
        (self.location.0 + (self.size as f32) - 1.0, self.location.1 + (self.size as f32) - 1.0)]
    }
}
