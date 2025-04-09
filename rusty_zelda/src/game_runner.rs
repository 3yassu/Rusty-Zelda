mod objects;
use objects::{felix::Felix, item, npc::{Enemy, Shopkeeper}, room_data::{self, HostileRoomData, RoomData}, world_runner::WorldCursor};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use rand::Rng;

const TILE_SIZE: u32 = 32;
const ROOM_WIDTH: u32 = 512;
const ROOM_HEIGHT: u32 = 352;
const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 11;

fn room_creation() -> Vec<RoomData>{ //make a vector containing ALL of the rooms lol...
    //LATER WILL IMPLEMENT FILE IO TO CHECK A TXT FILE.
    let mut vector: Vec<RoomData> = vec!();
    let room_contain = RoomData::Hostile(HostileRoomData::new(
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
        vec!(), //enemy vec
        vec!(), //item vec
    ));
    let room_two: RoomData =     RoomData::Hostile(HostileRoomData::new(
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
        vec!(Enemy::new(30, (128.0, 160.0)), Enemy::new(30, (256.0, 126.0)), Enemy::new(30, (350.0, 180.0))), //enemy vec
        vec!(),
    ));
	
	let room_three: RoomData =     RoomData::Hostile(HostileRoomData::new(
        vec![
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
           vec![1, 2, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
           vec![1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
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
	vector.push(room_three);
    vector
}


struct GameRunner { 
    felix: Felix,
    world: WorldCursor
}

impl GameRunner {
    fn new(felix: Felix, world: WorldCursor) -> Self{
        Self{felix, world}
    }

    fn in_loading_zone(&mut self) -> Option<char> {
        let corners = [
            (self.felix.location.0, self.felix.location.1),
            (self.felix.location.0 + self.felix.size as f32 - 1.0, self.felix.location.1),
            (self.felix.location.0, self.felix.location.1 + self.felix.size as f32 - 1.0),
            (self.felix.location.0 + self.felix.size as f32 - 1.0, self.felix.location.1 + self.felix.size as f32 - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            let room_dungeon;
            match self.world.get_curr(){
                RoomData::Shop(shop) => room_dungeon = &shop.dungeon,
                RoomData::Hostile(hostile) => room_dungeon = &hostile.dungeon,
            }
            if room_dungeon[tile_y][tile_x] == 2 {
                if (cx < 64.0) {return Some('l'); //right loading zone... etc
                }else if (cx > 448.0) {return Some('r');
                }else if (cy < 64.0) {return Some('n');
                }else {return Some('s');}
            }
        }
        None
    }
}

pub fn run() -> Result <(), String> {
    let mut going = true;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rusty Zelda", ROOM_WIDTH, ROOM_HEIGHT) //might need to change this l8er
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut a = room_creation().into_iter();
    let Some(n) = a.next() else{panic!("a");};

    let fel = Felix::new(TILE_SIZE/2, (256.0, 272.0)); //hi fel
    let cursor = WorldCursor::new(n); 
    let mut player = GameRunner::new(fel, cursor);
    let Some(n) = a.next() else{panic!("a");}; //i feel like we need a better way to do this
    player.world.add_west(n);
	let Some(n) = a.next() else{panic!("a");};
    player.world.add_east(n);
    
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
	//check if felix DIED like a dog
	if player.felix.health_bar <= 0 { break 'running;}

        let mut keys = event_pump.keyboard_state();
        let room_dungeon;
        match player.world.get_curr(){
            RoomData::Shop(shop) => room_dungeon = &shop.dungeon,
            RoomData::Hostile(hostile) => room_dungeon = &hostile.dungeon,
        }

        let mut item_rect = Rect::new(0, 0, 0, 0); //sword
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::Space){
            item_rect = player.felix.use_hand_a();
        }

        let room_enemy;
        match player.world.get_curr(){
            RoomData::Shop(shop) => room_enemy = vec!(),
            RoomData::Hostile(hostile) => room_enemy = hostile.get_enemy_col(),
        }

        player.felix.move_felix(&mut keys, room_dungeon, &room_enemy, true, &mut canvas);

	if let Some(loading_zone) = player.in_loading_zone() {
            match loading_zone {
                'l' => {
			player.world.traverse_west();
			player.world.set_connector();
			player.felix.location.0 = 432.0; player.felix.location.1 = 176.0;
                }
                'r' => {
			player.world.traverse_east();
			player.world.set_connector();
			player.felix.location.0 = 80.0; player.felix.location.1 = 176.0;
                }
                'n' => {
                    	player.world.traverse_north();
			player.world.set_connector();
			player.felix.location.0 = 256.0; player.felix.location.1 = 272.0;
                }
                's' => {
                    	player.world.traverse_south();
			player.world.set_connector();
			player.felix.location.0 = 256.0; player.felix.location.1 = 80.0;
                }
                _ => {}
            }
        }
	
        canvas.set_draw_color(Color::RGB(0,0,0)); //draws room
        canvas.clear();
        for y in 0..MAP_HEIGHT{
            for x in 0..MAP_WIDTH{
                let two_room_dungeon;
                match player.world.get_curr(){
                    RoomData::Shop(shop) => two_room_dungeon = &shop.dungeon,
                    RoomData::Hostile(hostile) => two_room_dungeon = &hostile.dungeon,
                }
                let tile = two_room_dungeon[y][x];
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

        canvas.set_draw_color(Color::RGB(255, 179, 26));
        let _ = canvas.fill_rect(player.felix.rect());

        canvas.set_draw_color(Color::RGB(0, 0, 0)); 
        let _ = canvas.fill_rect(item_rect);
	if let RoomData::Hostile(hostile) = player.world.get_curr_mut(){
		canvas.set_draw_color(Color::RGB(0, 0, 0)); //enemy color; change eventually
        let mut x: usize = 0;
        if (rand::rng().random_range(0..=10) == 1) {going = false;}
        let enemy_len = hostile.enemies.len();
        let mut remove_vec = vec!();
        for i in 0..enemy_len{
            let living: bool;
            if let Some(d) = &mut player.felix.hand.0{
                if let Some(b) = d.get_col(){
                    living = hostile.enemies[i].move_enemy(&hostile.dungeon, true, going, &vec!(b));
                }else{living = hostile.enemies[i].move_enemy(&hostile.dungeon, true, going, &vec!());}
            }else{living = hostile.enemies[i].move_enemy(&hostile.dungeon, true, going, &vec!());}
            if !living {remove_vec.push(i)}
            let _ = canvas.fill_rect(hostile.enemies[i].rect());
        }
        for dead in remove_vec {hostile.enemies.remove(dead);}
        going = true;
	}
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
 }
