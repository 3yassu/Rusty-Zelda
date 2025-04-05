//item=> struct with -id -animation -cost -collision

#[derive(Debug)]
pub struct Item {
    id: u32,
    //animation: links to graphics
    cost: i32,
    collision: bool,
    location: (Option<i32>, Option<i32>)
}

impl Item{
    pub fn new(id: u32, cost: i32, collision: bool, location: (Option<i32>, Option<i32>)) -> Self { 
        Self{id, cost, collision, location}
    }

    pub fn use((i32, i32)){
        match id {//only bare-bones implementing essential items
            0 => println!("use sword"), 
            3 => println!("use shield"),
            5 => println!("use boomerang"),
            7 => println!("use bomb"),
            8 => println!("use bow"),
            9 => println!("use arrow-- decrement rupee counter!"),
            16 => println!("use life potion-- fully restore hearts!"),
            30 => println!("use compass-- shows location of triforce fragment!"),
            31 => println!("use dungeon map-- shows each room of the dungeon!"),
            32 => println!("use small key-- unlocks Locked Doors!"),
            33 => println!("use triforce fragment~"),
            40 => println!("recovery heart-- heals 1 heart"),
            41 => println!("heart container-- increases max hearts by 1"),
            42 => println!("clock-- freezes all enemies"),
            43 => println!("rupee -- increments rupee counter")
        }
    }
}

fn main(){
    let my_item = Item::new(0, 0, true, (Some(0), Some(0)));
    print!("{:?}",my_item);
}
