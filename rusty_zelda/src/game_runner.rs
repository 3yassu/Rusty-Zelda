mod objects;

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

fn room_creation(){
    //initialize spawn room and room data
    let room1data = RoomData::Hostile(HostileRoomData::new(
        player_spawn: (512.0 - 8.5*32.0, 352.0 - 1.5 * 32.0); //this is shitty
        dungeon: Vec<Vec<u8>>  = vec! [
           vec! [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //add vec! to each line eyassu
           vec! [1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 1, 1, 1, 1, 1, 1],
           vec! [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec! [1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
           vec! [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec! [1, 2, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 2, 1],
           vec! [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec! [1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
           vec! [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
           vec! [1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1],
           vec! [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        enemy_vec = vec![];
        item_vec = vec![];
    ));
}

let world = WorldCursor::new(room1data);

struct Player { //could be in felix 
    x: f32, y: f32, speed: f32, size: u32,
}

impl Player {
    fn rect(&self) -> Rect{
        Rect::new(self.x as i32, self.y as i32, self.size, self.size)
    }
    fn in_collision(x: f32, y:f32, size: f32) -> bool {
        let corners = [
            (x, y),
            (x + size - 1.0, y),
            (x, y + size - 1.0),
            (x + size - 1.0, y + size - 1.0),
        ];
        for (cx, cy) in corners {
            let tile_x = (cx/TILE_SIZE as f32) as usize;
            let tile_y = (cy/TILE_SIZE as f32) as usize;
            if world.get_curr().dungeon[tile_y][tile_x] % 2 == 1 {return true};
        }
        false
    }
}

fn main() -> Result <(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rusty Zelda", ROOM_WIDTH, ROOM_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut player = Player {
        x: TILE_SIZE as f32 * 2.0, y: TILE_SIZE as f32 * 2.0,
        speed: 2.0, size: TILE_SIZE / 2,
    };

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

        let new_x = player.x + dx; let new_y = player.y + dy;

        if !in_collision(new_x, player.y, player.size as f32) {
            player.x = new_x;
        }
        if !in_collision(player.x, new_y, player.size as f32){
            player.y = new_y;
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        for y in 0..MAP_HEIGHT{
            for x in 0..MAP_WIDTH{
                let tile = world.get_curr().dungeon[y][x];
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
        let _ = canvas.fill_rect(player.rect());
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
 }
