use core::ptr::NonNull; //EWWW NONNULL POINTER BLEH :P
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
pub struct RoomData{//room item info & image data
    information: i32,
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