use std::ptr::NonNull; //EWWW NONNULL POINTER BLEH :P
use super::room_data;
use super::npc;

#[derive(Debug)] // <- yk what this does (derive debug allows you to say dbg!(item) and print its info, you could also impl debug trait...
struct Room{ //Room struct has pointers (Connectors) to rooms in all 6 directions
    pub data: room_data::RoomData, //(has data{room item info & image data}
    north: Connector,
    east: Connector,
    south: Connector,
    west: Connector,
    up: Connector, //might change to above?
    down: Connector //might change to below?
}
impl Room{//impl new for Room (debating on wanting to initialize with no data or give it data?)
    pub fn new(data: room_data::RoomData) -> Self{//if I don't include data add functions that add the individual data pieces
        Self{data, north: None, south: None, east: None, west: None, up: None, down: None}
    }
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
impl WorldCursor{
    pub fn set_connector(&mut self){//lets you put the connector at a position
        self.current = self.traverse;
    }
    pub fn connect_north(&mut self){
        unsafe{
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).north{
                        None => (*ptr.as_ptr()).north = self.current,
                        _ => panic!("RoomOverflowError: [WorldCursor].connect_north() Tried to undo connection!")
                    }
                    if let Some(current) = self.current{
                        (*current.as_ptr()).south = Some(*ptr);
                    }
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn connect_south(&mut self){
        unsafe{
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).south{
                        None => (*ptr.as_ptr()).south = self.current,
                        _ => panic!("RoomOverflowError: [WorldCursor].connect_north() Tried to undo connection!")
                    }
                    if let Some(current) = self.current{
                        (*current.as_ptr()).north = Some(*ptr);
                    }
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn connect_east(&mut self){
        unsafe{
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).east{
                        None => (*ptr.as_ptr()).east = self.current,
                        _ => panic!("RoomOverflowError: [WorldCursor].connect_north() Tried to undo connection!")
                    }
                    if let Some(current) = self.current{
                        (*current.as_ptr()).west = Some(*ptr);
                    }
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn connect_west(&mut self){
        unsafe{
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).west{
                        None => (*ptr.as_ptr()).west = self.current,
                        _ => panic!("RoomOverflowError: [WorldCursor].connect_north() Tried to undo connection!")
                    }
                    if let Some(current) = self.current{
                        (*current.as_ptr()).east = Some(*ptr);
                    }
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn connect_up(&mut self){
        unsafe{
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).up{
                        None => (*ptr.as_ptr()).up = self.current,
                        _ => panic!("RoomOverflowError: [WorldCursor].connect_north() Tried to undo connection!")
                    }
                    if let Some(current) = self.current{
                        (*current.as_ptr()).down = Some(*ptr);
                    }
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
    pub fn connect_down(&mut self){
        unsafe{
            match &mut self.traverse{
                Some(ptr) => {
                    match &mut (*ptr.as_ptr()).down{
                        None => (*ptr.as_ptr()).down = self.current,
                        _ => panic!("RoomOverflowError: [WorldCursor].connect_north() Tried to undo connection!")
                    }
                    if let Some(current) = self.current{
                        (*current.as_ptr()).up = Some(*ptr);
                    }
                }
                None => ()//panic!("ERROR: [WorldCursor] self.add_south(), tried to add south to None")
            }
        }
    }
}
impl WorldCursor{ //instantiation and adding code
    pub fn new(current_data: room_data::RoomData) -> Self{
        unsafe{
            let room = NonNull::new_unchecked(Box::into_raw(Box::new(Room::new(current_data))));
            Self{current: Some(room), traverse: Some(room)}            
        }
    }
    pub fn add_north(&mut self, room_data: room_data::RoomData){
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
    pub fn add_south(&mut self, room_data: room_data::RoomData){ //CHECK add_north
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
    pub fn add_east(&mut self, room_data: room_data::RoomData){ //CHECK add_north
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
    pub fn add_west(&mut self, room_data: room_data::RoomData){ //CHECK add_north
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
    pub fn add_up(&mut self, room_data: room_data::RoomData){ //CHECK add_north
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
    pub fn add_down(&mut self, room_data: room_data::RoomData){ //CHECK add_north
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
    pub fn move_x(&mut self, x: i32, at_current: bool){
        if at_current == true {
            self.traverse = self.current;
        }
        match x > 0 {
            true => {
                for _ in 0..x {self.traverse_east();}
            },
            false => {
                for _ in 0..(-x) {self.traverse_west();}
            }
        }
    }
    pub fn move_y(&mut self, x: i32, at_current: bool){
        if at_current == true {
            self.traverse = self.current;
        }
        match x > 0 {
            true => {
                for _ in 0..x {self.traverse_north();}
            },
            false => {
                for _ in 0..(-x) {self.traverse_south();}
            }
        }
    }
    pub fn move_z(&mut self, x: i32, at_current: bool){
        if at_current == true {
            self.traverse = self.current;
        }
        match x > 0 {
            true => {
                for _ in 0..x {self.traverse_up();}
            },
            false => {
                for _ in 0..(-x) {self.traverse_down();}
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
impl WorldCursor{
    pub fn get_curr(&mut self) -> &Vec<Vec<u8>>{
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Shop(shop) => &shop.dungeon,
                        room_data::RoomData::Hostile(hostile) => &hostile.dungeon,
                    }
                },
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
    }
    pub fn get_curr_mut(&mut self) -> &mut Vec<Vec<u8>>{
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Shop(shop) => &mut shop.dungeon,
                        room_data::RoomData::Hostile(hostile) => &mut hostile.dungeon,
                    }
                },
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
    }
    pub fn access_spawn(&mut self) -> &mut (f32, f32){
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Shop(shop) => shop.access_spawn(),
                        room_data::RoomData::Hostile(hostile) => hostile.access_spawn(),
                    }
                },
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
    }
    pub fn get_enemy(&mut self) -> &mut Vec<npc::Enemy>{
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Shop(shop) => panic!("Ugh. (self.get_enemy())"),
                        room_data::RoomData::Hostile(hostile) => hostile.get_enemy(),
                    }
                }
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
    }
    pub fn get_enemy_ref(&mut self) -> &Vec<npc::Enemy>{
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Shop(shop) => panic!("Ugh. (self.get_enemy())"),
                        room_data::RoomData::Hostile(hostile) => hostile.get_enemy_ref(),
                    }
                }
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
    }
    pub fn is_hostile(&mut self) -> bool{
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Shop(shop) => false,
                        room_data::RoomData::Hostile(hostile) => true,
                    }
                }
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
    }
    pub fn move_enemy(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, keep_going: bool){ //WANT TO GET OUT OF HERE PLEASE!
        let mut x: &mut Vec<npc::Enemy> = &mut vec!();
        unsafe{
            match self.current{
                Some(room) => {
                    match &mut (*room.as_ptr()).data{
                        room_data::RoomData::Hostile(hostile) => x = hostile.get_enemy(),
                        _ => ()
                    }
                }
                None => panic!("[WorldCursor].get_curr() tried to get None....")
            }
        }
        for i in x{
            let _ = canvas.fill_rect(i.rect());
            i.move_enemy(self.get_curr(),true, keep_going);
        }
    }
}
impl Drop for WorldCursor{ //it gets mad if I didn't implement a drop function :( that's not always the case ig it's just STUPID
    fn drop(&mut self){
        self.clear();//recursively remove from the traverse
    } //even if traverse didn't get removed it goes out of scope HERE and gets DROPPED muahhahha
}


#[cfg(test)]
mod test {
    #[test]
    fn tester(){ //TEST
        /*
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
        */
    }
}
