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
        dimensions: (ROOM_WIDTH, ROOM_HEIGHT),
        player_spawn: (512.0 - 8.5*32.0, 352.0 - 1.5 * 32.0); //this is shitty
        dungeon: [[u8; dimensions.0]; dimensions.1] = [



        ];
    ));
    let room1: Room = {room1data};
}
