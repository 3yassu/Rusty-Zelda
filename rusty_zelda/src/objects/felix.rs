mod item;
struct Felix{
    inventory: Vec<Item>,
    health_bar: u32,
    rupee_balance: i32,
    hand: (Option<Item>, Option<Item>),
    location: (i32, i32),
}
