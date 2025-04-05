//Nothing for now :)
#[derive(Debug)]
pub struct ShopRoomData{//room item info & image data
    dimensions: (usize, usize),
    dungeon: [[u8; dimensions.0]; dimensions.1],
    shopkeeper: Shopkeeper::new() //shopkeeper handles items, etc
}

#[derive(Debug)]
pub struct HostileRoomData{
    dimensions: (usize, usize),
    dungeon: [[u8; dimensions.0]; dimensions.1],
    enemies: Vec<Enemy::new()>,
    items: Vec<Item::new()>
}
