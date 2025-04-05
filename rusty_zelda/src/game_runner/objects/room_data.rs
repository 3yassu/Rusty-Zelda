//Nothing for now :)
#[derive(Debug)]
pub struct ShopRoomData{//room item info & image data
    //dimensions: (i32, i32),
    shopkeeper: Shopkeeper::new() //shopkeeper handles items, etc
}
#[derive(Debug)]
pub struct HostileRoomData{
    //dimensions: (i32, i32),
    enemies: Vec<Enemy::new()>,
    items: Vec<Item::new()>,
    teleports: Vec<(i32, i32)> //special teleports in the middle of room
}