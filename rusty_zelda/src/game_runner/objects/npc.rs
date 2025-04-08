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
    //color: (u8, u8, u8)
    //animations
}

impl Enemy { //new or instantiation
    pub fn new(id: u32, location: (f32, f32)) -> Self {
        match id{
            /*
            0 => {Self::new_armos(location)},
            1 => {Self::new_boulder(location)},
            2 => {Self::new_ghini(location)} OR Self::new_gaeny(location)} ,
            3 => {Self::new_leever(location, "R")},
            4 => {Self::new_leever(location, "B")},
            5 => {Self::new_lynel(location, "R")},
            6 => {Self::new_lynel(location, "B")},
            7 => {Self::new_moblin(location, "R")},
            8 => {Self::new_moblin(location, "B")},
            9 => {Self::new_oktorok(location, "R")},
            10 => {Self::new_oktorok(location, "B")},
            11 => {Self::new_peahat(location)},
            12 => {Self::new_river_zora(location)},
            13 => {Self::new_tektike(location, "R")},
            14 => {Self::new_tektike(location, "R")},

            20 => {Self::new_blade_trap(location)},
            21 => {Self::new_bubble(location, "R")},
            22 => {Self::new_bubble(location, "B")},
            23 => {Self::new_bubble(location, "F")},
            24 => {Self::new_darknut(location, "R")},
            25 => {Self::new_darknut(location, "B")},
            26 => {Self::new_gel(location)},
            27 => {Self::new_gibdo(location)},
            28 => {Self::new_goriya(location, "R")},
            29 => {Self::new_goriya(location, "B")},
            */
            30 => {Self::new_keese(location)},
            /*
            31 => {Self::new_lanmola(location, "R")},
            32 => {Self::new_lanmola(location, "B")},
            33 => {Self::new_like_like(location)},
            34 => {Self::new_moldorm(location)},
            35 => {Self::new_patra(location)},      -|
                                                     |--Might make into one patra enemy with weird rect method?
            36 => {Self::new_patra_baby(location)}, -|
            37 => {Self::new_pols_voice(location)},
            38 => {Self::new_rope(location, "R")},
            39 => {Self::new_rope(location, "F")},
            */
            40 => {Self::new_stalfos(location)},
            /* 
            41 => {Self::new_stone_statue(location)},
            42 => {Self::new_vire(location)},
            43 => {Self::new_wallmaster(location)},
            44 => {Self::new_wizzrobe(location, "R")},
            45 => {Self::new_wizzrobe(location, "B")},
            46 => {Self::new_zol(location)},
            */
            _ => panic!("called new() with unimplemented ID"),
        }
    }
    //enemy ai -- attacks need to be based on enemy id/type.
    pub fn new_keese(location: (f32, f32)) -> Self{ //quick debug functions
        Self{item_on_kill: item::Item::new(100, 0, None, false, None, 0, 0, 0.0), id: 30, collision: true, hp: 1, ignore_room_collision: true, location, size: 8, speed: 2.0, delta_pos: (0.0, 0.0)}
    }
    pub fn new_stalfos(location: (f32, f32)) -> Self{ //quick debug functions
        Self{item_on_kill: item::Item::new(100, 0, None, false, None, 0, 0, 0.0), id: 40, collision: true, hp: 2, ignore_room_collision: false, location, size: 16, speed: 4.0, delta_pos: (0.0, 0.0)}
    }
}
impl Enemy{
    //enemy movement
    pub fn move_enemy(&mut self, world_dungeon: &Vec<Vec<u8>>, can_move: bool, keep_going: bool, item_corner: &Vec<[(f32, f32); 4]>) -> bool{
            let possible_x: f32 = rand::rng().random_range(0..=2) as f32;
            let possible_y: f32 = rand::rng().random_range(0..=2) as f32;
        match self.id{
            30 => {self.move_keese(possible_x, possible_y, world_dungeon, can_move, keep_going, item_corner)},
            40 => {self.move_stalfos(possible_x, possible_y, world_dungeon, can_move, keep_going, item_corner)},
            _ => (true)
        }
    }
    fn move_keese(&mut self, x: f32, y: f32, world_dungeon: &Vec<Vec<u8>>, can_move: bool, keep_going: bool, item_corner: &Vec<[(f32, f32); 4]>) -> bool{
        if !can_move {return true;}
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
        match self.enem_collision(new_x, self.location.1, self.size as f32, world_dungeon, item_corner){
            Some(a) => if !a{self.location.0 = new_x;},
            None => return false,
        }
        match self.enem_collision(self.location.0, new_y, self.size as f32, world_dungeon, item_corner){
            Some(a) => if !a{self.location.1 = new_y},
            None => return false,
        }
        true
    }
    fn move_stalfos(&mut self, x: f32, y: f32, world_dungeon: &Vec<Vec<u8>>, can_move: bool, keep_going: bool, item_corner: &Vec<[(f32, f32); 4]>) -> bool{
        if !can_move {return true;}
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
        match self.enem_collision(new_x, self.location.1, self.size as f32, world_dungeon, item_corner){
            Some(a) => if !a{self.location.0 = new_x;},
            None => return false,
        }
        match self.enem_collision(self.location.0, new_y, self.size as f32, world_dungeon, item_corner){
            Some(a) => if !a{self.location.1 = new_y},
            None => return false,
        }
        true
    }
    pub fn rect(&self) -> Rect{
        Rect::new(self.location.0 as i32, self.location.1 as i32, self.size, self.size)
    }
    pub fn enem_collision(&mut self, x: f32, y:f32, size: f32, world_dungeon: &Vec<Vec<u8>>, item_corner: &Vec<[(f32, f32); 4]>) -> Option<bool>{
        let corners: [(f32, f32); 4]  = [
            (x, y),
            (x + size - 1.0, y),
            (x, y + size - 1.0),
            (x + size - 1.0, y + size - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            if world_dungeon[tile_y][tile_x] != 0 {return Some(true)};
            for enemy in item_corner{
                if (cx >= enemy[0].0 && cx <= enemy[1].0) 
                && 
                (cy >= enemy[0].1 && cy <= enemy[2].1){
                println!("AHH");
                self.hp -= 1;
                if self.hp == 0 {return None;}
                return Some(true);
                }
            }
        }
        Some(false)
    }
    pub fn get_col(&self) -> [(f32, f32); 4]{
        [self.location, 
        (self.location.0 + (self.size as f32) - 1.0, self.location.1), 
        (self.location.0, self.location.1 + (self.size as f32) - 1.0), 
        (self.location.0 + (self.size as f32) - 1.0, self.location.1 + (self.size as f32) - 1.0)]
    }
}
