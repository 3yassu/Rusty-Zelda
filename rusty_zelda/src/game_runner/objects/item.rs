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

    pub fn on_pickup(){ //on pick-up of essential items
        match id {//only bare-bones implementing essential items
            0 => println!("add sword to inventory"), 
            3 => println!("add shield to inventory"),
            5 => println!("add boomerang to inventory"),
            7 => println!("add bomb to inventory"),
            8 => println!("add bow to inventory"),
            9 => println!("add arrow to inventory"),
            16 => println!("add life potion to inventory"),
            30 => println!("modifies ui to show location of fragment"),
            31 => println!("modifies ui to show dungeon room layout"),
            32 => println!("add small key to inventory"),
            33 => println!("add triforce frag to inventory"),
            40 => println!("heals 1 heart"),
            41 => println!("increases max hearts by 1"),
            42 => println!("freezes all enemies"),
            43 => println!("increments rupee counter"),
            _ => println!("unimplemented item...")
        }
    }

    pub fn use_item(location: (i32, i32)) { //use weapons
        match id {
            0 => println!("use sword"),
            3 => println!("use shield"),
            5 => println!("use boomerang"),
            7 => println!("use bomb"), //if bomb in inventory
            8 => println!("use bow"), //if rupee > 0 AND arrows in inventory
            16 => println!("use life potion- fully restores hearts"),
            32 => println!("use small key"), //may be incorrect behavior
            _ => println!("unimplemented item...")
        }
    }
}

fn main(){
    let my_item = Item::new(0, 0, true, (Some(0), Some(0)));
    print!("{:?}",my_item);
}
