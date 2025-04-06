mod objects;
use objects::{world_runner::WorldCursor, room_data::{RoomData, HostileRoomData}};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::vec::IntoIter;

const TILE_SIZE: u32 = 32;
const ROOM_WIDTH: u32 = 512;
const ROOM_HEIGHT: u32 = 352;
const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 11;

fn room_creation() -> Vec<RoomData>{ //make a vector containing ALL of the rooms lol...
    //LATER WILL IMPLEMENT FILE IO TO CHECK A TXT FILE.
    let mut vector: Vec<RoomData> = vec!();
    let room_contain = RoomData::Hostile(HostileRoomData::new(
        (256.0, 272.0), //this var is completely useless now fun fact
        vec![
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 2, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 2, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //no overworld access ... yet!
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ],
        vec!(),
        vec!(),
    ));
    let room_two: RoomData =     RoomData::Hostile(HostileRoomData::new(
        (0.0, 0.0), //change spawn l8ter
        vec![
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ],
        vec!(),
        vec!(),
    ));
    vector.push(room_contain); //LOLL.. for now implementation is to do this as many times as possible
    vector.push(room_two);
    vector
}


struct Player { //could be in felix 
    x: f32, y: f32, speed: f32, size: u32,
    world: WorldCursor
}

impl Player {
    fn rect(&self) -> Rect{
        Rect::new(self.x as i32, self.y as i32, self.size, self.size)
    }
    fn in_collision(&mut self, x: f32, y:f32, size: f32) -> bool {
        let corners = [
            (x, y),
            (x + size - 1.0, y),
            (x, y + size - 1.0),
            (x + size - 1.0, y + size - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            if self.world.get_curr()[tile_y][tile_x] % 2 == 1 {return true};
        }
        false
    }
    fn in_loading_zone(&mut self) -> Option<char> {
        let corners = [
            (self.x, self.y),
            (self.x + self.size as f32 - 1.0, self.y),
            (self.x, self.y + self.size as f32 - 1.0),
            (self.x + self.size as f32 - 1.0, self.y + self.size as f32 - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            if self.world.get_curr()[tile_y][tile_x] == 2 {
                if (cx < 64.0) {return Some('l'); //right loading zone... etc
                }else if (cx > 448.0) {return Some('r');
                }else if (cy < 64.0) {return Some('n');
                }else {return Some('s');}
            }
            
        }
        None
    }
}

pub fn bain() -> Result <(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rusty Zelda", ROOM_WIDTH, ROOM_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut a = room_creation().into_iter();
    let Some(n) = a.next() else{panic!("a");};
    let mut player = Player {
        x: 256.0, y: 272.0,
        speed: 2.0, size: TILE_SIZE / 2,
        world: WorldCursor::new(n)
    };
    let Some(n) = a.next() else{panic!("a");};
    player.world.add_west(n);

    'running: loop{
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        //this could maybe be migrated to a Player function
        let keys = event_pump.keyboard_state();
        let mut dx = 0.0; let mut dy = 0.0; 
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Left){
            dx -= player.speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Right){
            dx += player.speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Up){
            dy -= player.speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Down){
            dy += player.speed;
        }

	if let Some(loading_zone) = player.in_loading_zone() {
            match loading_zone {
                'l' => {
			player.world.traverse_west();
			player.world.set_connector();
			player.x = 432.0; player.y = 176.0;
                }
                'r' => {
			player.world.traverse_east();
			player.world.set_connector();
			player.x = 80.0; player.y = 176.0;
                }
                'n' => {
                    	player.world.traverse_north();
			player.world.set_connector();
			player.x = 256.0; player.y = 272.0;
                }
                's' => {
                    	player.world.traverse_south();
			player.world.set_connector();
			player.x = 256.0; player.y = 80.0;
                }
                _ => {}
            }
        }

        let new_x = player.x + dx; let new_y = player.y + dy;

        if !player.in_collision(new_x, player.y, player.size as f32) {
            player.x = new_x;
        }
        if !player.in_collision(player.x, new_y, player.size as f32){
            player.y = new_y;
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        unsafe{
        for y in 0..MAP_HEIGHT{
            for x in 0..MAP_WIDTH{
                let tile = player.world.get_curr()[y][x];
                let color = match tile{
                    0 => Color::RGB(255, 51, 0),
                    1 => Color::RGB(128, 0, 0),
                    2 => Color::RGB(26, 13, 0),
                    3 => Color::RGB(153, 0, 51),
                    _ => Color::RGB(0, 204, 0),
                };
                canvas.set_draw_color(color);
                let _ = canvas.fill_rect(Rect::new(
                        (x as i32) * TILE_SIZE as i32,
                        (y as i32) * TILE_SIZE as i32,
                        TILE_SIZE, TILE_SIZE,
                ));
            }
        }
        }

        canvas.set_draw_color(Color::RGB(255, 179, 26));
        let _ = canvas.fill_rect(player.rect());
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
 }
