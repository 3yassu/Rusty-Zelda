use std::ptr::NonNull; //EWWW NONNULL POINTER BLEH :P
mod item;
mod npc;

#[derive(Debug)] // <- yk what this does (derive debug allows you to say dbg!(item) and print its info, you could also impl debug trait...
struct Room{ //Room struct has pointers (Connectors) to rooms in all 6 directions
    pub data: RoomData, //(has data{room item info & image data}
    north: Connector,
    east: Connector,
    south: Connector,
    west: Connector,
    up: Connector, //might change to above?
    down: Connector //might change to below?
}
impl Room{//impl new for Room (debating on wanting to initialize with no data or give it data?)
    pub fn new(data: RoomData) -> Self{//if I don't include data add functions that add the individual data pieces
        Self{data, north: None, south: None, east: None, west: None, up: None, down: None}
    }
}
#[derive(Debug)]
pub struct ShopRoomData{//room item info & image data
    //dimensions: (i32, i32),
    shopkeeper: Shopkeeper::new() //shopkeeper handles items, etc
}

#[derive(Debug)]
pub struct RoomData{
    //dimensions: (i32, i32),
    enemies: Vec<Enemy::new()>,
    items: Vec<Item::new()>,
    teleports: Vec<(i32, i32)> //special teleports in the middle of room
}
type Connector = Option<NonNull<Room>>;//connector type is Option of a NonNull of a Room;
#[derive(Debug)]
pub struct WorldCursor{ 
    current: Connector, //current => keep track of player position
    traverse: Connector //traverse => Cursor for world instantiation (probably set to none after fully created)
}
impl WorldCursor{ //traverse implementation (I wanted to seperate for readability)
    pub fn traverse_north(&mut self){
        unsafe{ //dereferencing ptr's are unsafe so it needs this 
            match self.traverse.take(){ //traverse is gonna change so you can use .take() {takes the self.traverse info and sets self.traverse to None}
                Some(ptr) => self.traverse = (*ptr.as_ptr()).north, //sets self.traverse to it's north (have to do weird ptr stuff...)
                None => ()//panic!("ERROR: [WorldCursor] self.traverse_north(), tried to go north on None")
            }; // Debating on adding a paniic if you try to traverse while on None ^^ (if not will change to if let format)
        }//MAYBE: -> change to if let Some(ptr) = self.traverse.take{self.traverse = (*ptr.as_ptr()).north;}
    }
    pub fn traverse_south(&mut self){ //CHECK traverse_north
        unsafe{
            match self.traverse.take(){ 
                Some(ptr) => self.traverse = (*ptr.as_ptr()).south,
                None => ()//panic!("ERROR: [WorldCursor] self.traverse_north(), tried to go north on None")
            };
        }
    }
    pub fn traverse_east(&mut self){//CHECK traverse_north
        unsafe{
            match self.traverse.take(){
                Some(ptr) => self.traverse = (*ptr.as_ptr()).east,
                None => ()//panic!("ERROR: [WorldCursor] self.traverse_north(), tried to go north on None")
            };
        }
    }
    pub fn traverse_west(&mut self){//CHECK traverse_north
        unsafe{
            match self.traverse.take(){
                Some(ptr) => self.traverse = (*ptr.as_ptr()).west,
                None => ()//panic!("ERROR: [WorldCursor] self.traverse_north(), tried to go north on None")
            };
        }
    }
    pub fn traverse_up(&mut self){//CHECK traverse_north
        unsafe{
            match self.traverse.take(){
                Some(ptr) => self.traverse = (*ptr.as_ptr()).up,
                None => ()//panic!("ERROR: [WorldCursor] self.traverse_north(), tried to go north on None")
            };
        }
    }
    pub fn traverse_down(&mut self){//CHECK traverse_north
        unsafe{
            match self.traverse.take(){
                Some(ptr) => self.traverse = (*ptr.as_ptr()).down,
                None => ()//panic!("ERROR: [WorldCursor] self.traverse_north(), tried to go north on None")
            };
        }
    }
}
impl WorldCursor{ //instantiation and adding code
    pub fn new(current_data: RoomData) -> Self{
        unsafe{
            let room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(current_data))));
            Self{current: Some(room), traverse: Some(room)}            
        }
    }
    pub fn add_north(&mut self, room_data: RoomData){
        unsafe{// Though NonNull::new() isn't unsafe NonNull::new_unchecked is (It's faster as it bypasses safety checks just don't be stupid)
            let new_room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(room_data)))); //create new_room with room info (debating on giving it a room instead of room_data)
            match &mut self.traverse{ //mutable reference for match since I wanna keep self.traverse as it is
                Some(ptr) => { //if some set that things north to Some(new_room)
                    match &mut (*ptr.as_ptr()).north{
                        None => (*ptr.as_ptr()).north = Some(new_room),
                        _ => panic!("RoomOverflowError: [WorldCursor].add_north() Tried to overwrite room!")
                    }
                    (*new_room.as_ptr()).south = Some(*ptr); //and then sets the new_room's south to doubly link it :D
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_north(), tried to add north to None")
            }// Debating on adding a paniic if you try to traverse while on None ^^ (if not will change to if let format)
        }
    }
    pub fn add_south(&mut self, room_data: RoomData){ //CHECK add_north
        unsafe{
            let new_room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(room_data))));
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).south{
                        None => (*ptr.as_ptr()).south = Some(new_room),
                        _ => panic!("RoomOverflowError: [WorldCursor].add_south() Tried to overwrite room!")
                    }
                    (*new_room.as_ptr()).north = Some(*ptr);
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn add_east(&mut self, room_data: RoomData){ //CHECK add_north
        unsafe{
            let new_room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(room_data))));
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).east{
                        None => (*ptr.as_ptr()).east = Some(new_room),
                        _ => panic!("RoomOverflowError: [WorldCursor].add_east() Tried to overwrite room!")
                    }
                    (*new_room.as_ptr()).west = Some(*ptr);
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn add_west(&mut self, room_data: RoomData){ //CHECK add_north
        unsafe{
            let new_room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(room_data))));
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).west{
                        None => (*ptr.as_ptr()).west = Some(new_room),
                        _ => panic!("RoomOverflowError: [WorldCursor].add_west() Tried to overwrite room!")
                    }
                    (*new_room.as_ptr()).east = Some(*ptr);
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn add_up(&mut self, room_data: RoomData){ //CHECK add_north
        unsafe{
            let new_room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(room_data))));
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).up{
                        None => (*ptr.as_ptr()).up = Some(new_room),
                        _ => panic!("RoomOverflowError: [WorldCursor].add_up() Tried to overwrite room!")
                    }
                    (*new_room.as_ptr()).down = Some(*ptr);
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn add_down(&mut self, room_data: RoomData){ //CHECK add_north
        unsafe{
            let new_room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(room_data))));
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).down{
                        None => (*ptr.as_ptr()).down = Some(new_room),
                        _ => panic!("RoomOverflowError: [WorldCursor].add_down() Tried to overwrite room!")
                    }
                    (*new_room.as_ptr()).up = Some(*ptr);
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
}
impl WorldCursor{
    fn clear(&mut self){ //WANNA CHANGE TO ITERATIVE (also add functionanitly to put some metadata in a file for save states)
        self.current = None; //makes current None so it doesn't become a dangling pointer
        unsafe{ //dereferencing raw pointers is unsafe!!!
            if self.traverse.is_some(){ //Not doing anything if None 😋! (only for case of self.traverse being None)
                let mut direction_vec: Vec<Connector> = vec![]; //Initialize vector to hold all direction that are some (it yelled at me for not adding = vec![] 🙁)
                direction_vec.push(self.traverse);
                loop { //create scope to implicitly drop box at end
                    let mut con_vec: Vec<Connector> = vec![];//convec to deal with borrow checker
                    for dir in direction_vec{
                        let boxed = Box::from_raw(dir.unwrap().as_ptr()); //Box up the pointer [and it's item] (so rust does all the dealocation for us LOL)
                        if let Some(north) = boxed.north{ //if north is not None
                            (*north.as_ptr()).south = None; //set the norths south to None (so there's no dangling ponter)
                            con_vec.push(boxed.north) //push the north val to convec
                        }
                        if let Some(south) = boxed.south{ //SEE NORTH >:(
                            (*south.as_ptr()).north = None;
                            con_vec.push(boxed.south);
                        }
                        if let Some(east) = boxed.east{ //SEE NORTH >:(
                            (*east.as_ptr()).west = None;
                            con_vec.push(boxed.east);
                        }
                        if let Some(west) = boxed.west{ //SEE NORTH >:(
                            (*west.as_ptr()).east = None;
                            con_vec.push(boxed.west);
                        }
                        if let Some(up) = boxed.up{ //SEE NORTH >:(
                            (*up.as_ptr()).down = None;
                            con_vec.push(boxed.up);
                        }
                        if let Some(down) = boxed.down{ //SEE NORTH >:(
                            (*down.as_ptr()).up = None;
                            con_vec.push(boxed.down);
                        }
                    }
                    if con_vec.is_empty(){break;}//if nothing was added to con_vec then break
                    else{direction_vec = con_vec;}//otherwise direction_vec set to con_vec
                } //THIS DEALOCATED THE BOX  L O L, we could not include the {} and add drop(boxed); your choice idk...
            }
        }        
    }
}
impl Drop for WorldCursor{ //it gets mad if I didn't implement a drop function :( that's not always the case ig it's just STUPID
    fn drop(&mut self){
        self.clear();//recursively remove from the traverse
    } //even if traverse didn't get removed it goes out of scope HERE and gets DROPPED muahhahha
}


fn main(){ //TEST
    let data = RoomData{information: 125};
    let mut worldy = WorldCursor::new(data);
    let new_data = RoomData{information: 15};
    worldy.add_north(new_data);
    let new_data = RoomData{information: 12};
    worldy.traverse_north();
    worldy.add_north(new_data);
    worldy.traverse_south();
    unsafe{println!("{}", (*worldy.current.unwrap().as_ptr()).data.information)};
    dbg!(&worldy);
}
