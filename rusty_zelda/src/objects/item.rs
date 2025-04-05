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
        match id //based on item id, functionality differs.
    }
}

fn main(){
    let my_item = Item::new(0, 0, true, (Some(0), Some(0)));
    print!("{:?}",my_item);
}
