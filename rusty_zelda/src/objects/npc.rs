mod item;

//two types of npc: shopkeeper and enemy

#[derive(Debug)]
pub struct Shopkeeper {
    items: Vec<item::new()>,
    //text_gen
    //item_text_gen?
    location: (i32, i32),
    collision: bool
}

impl Shopkeeper {
    pub fn new(items: Vec<item::new()>, location: (i32, i32), collision: bool) -> Self {
        Self{items, location, collision}
    }
    //shopkeeper and items lose visibility after felix has get_item animation
}

pub struct Enemy {
    item_on_kill: item::new(),
    id: u32,
    collision: bool,
    ignore_room_collision: bool,
    location: (i32, i32)
    //animations
}

impl Enemy {
    pub fn new(item_on_kill: item::new(), id: u32, collision: bool, ignore_room_collision: bool, location: (i32, i32) ) -> Self {
        Self{item_on_kill, id, collision, ignore_room_collision}
    }
    //enemy movement
    pub fn move_enemy(&mut self, x: i32, y: i32){
        self.location.0 +=x; self.location.1 += y;
    }
    //enemy ai -- attacks need to be based on enemy id/type. 
}
