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
    pub inventory: Vec<item::Item>,
    pub health_bar: u8, //2*(heart count)
    pub rupee_balance: i32,
    pub hand: (Option<item::Item>, Option<item::Item>),
    pub location: (f32, f32),
    pub size: u32,
    pub speed: f32,
}
impl Felix{
    pub fn new(size: u32, location: (f32, f32)) -> Self {
        Self{inventory: vec!(), health_bar: 6, rupee_balance: 0, location, hand: (Some(item::new(0,0,None,true,(None, None),8, 1)), None), size, speed: 2.0}
    }
    pub fn move_felix(&mut self, keys: sdl2::keyboard::KeyboardState<'_>, world_dungeon: &Vec<Vec<u8>>, enem_corner: &Vec<[(f32, f32); 4]>, can_move: bool){ //Keep in mind this function doesn't teleport felix but add's given position
        if !can_move {return;}
        let (mut dx, mut dy): (f32, f32) = (0.0, 0.0);
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Left){
            dx -= self.speed;
        }
        else if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Right){
            dx += self.speed;
        }
        else if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Up){
            dy -= self.speed;
        }
        else if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Down){
            dy += self.speed;
        }
        let (new_x, new_y) = (self.location.0 as f32 + dx, self.location.1 as f32 + dy);

        if !self.in_collision(new_x, self.location.1, self.size as f32, world_dungeon, enem_corner){
            self.location.0 = new_x;
        }
        if !self.in_collision(self.location.0, new_y, self.size as f32, world_dungeon, enem_corner){
            self.location.1 = new_y;
        }
    }

    pub fn use_hand_a(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
	//render sword for like 3 seconds
	if let Some(_) = self.hand.0{
		canvas.set_draw_color(Color::RGB(255, 255, 255)); 
		let _ = canvas.fill_rect(self.hand.0.as_mut().unwrap().rect(self.location));
	}

	//enable collision
	/*
        if let Some(item) = self.hand.0.as_mut(){
            item.use_item(self.location, &mut self.rupee_balance);
            if item.is_disposable(){
                item.count_remove();
                if item.count() == &Some(0) {
                    self.hand.0 = None;
                }
            }
        } */
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
    pub fn rect(&self) -> Rect{
        Rect::new(self.location.0 as i32, self.location.1 as i32, self.size, self.size)
    }
    fn in_collision(&mut self, x: f32, y:f32, size: f32, world_dungeon: &Vec<Vec<u8>>, enem_corner: &Vec<[(f32, f32); 4]>) -> bool {
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
		self.health_bar -= 1;
		println!("ouch");
                return true;
                }
            }
        }
        false
    }
}