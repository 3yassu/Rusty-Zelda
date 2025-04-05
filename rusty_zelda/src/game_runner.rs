mod objects;
pub struct GameRunner{
    fps: u32,
    PLAYER_SPEED: i32
    //graphics: SDL2 STUFF (WORRY AB LATER) 
}
impl GameRunner{
    pub fn new(fps: u32) -> Self {
        Self{fps}
    }
    pub fn update(&mut self, delta_time: u32){ //maybe make sub_functions for update to make code more readable
        //if Some(key) = GameRunner::detect_key(){
            //match key{
                //sdl2::Keyboard
            //}
        //}
    }
}
impl GameRunner{
    pub fn render(&mut self){ //maybe make sub_functions for render to make code more readable
        let two = 1+1;
    }
}