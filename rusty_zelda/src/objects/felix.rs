mod item;
pub struct Felix{ //Name of character (Fe)lix RedOx haha!
    inventory: Vec<Item>,
    health_bar: u8, //2*(heart count)
    rupee_balance: i32,
    hand: (Option<Item>, Option<Item>),
    location: (i32, i32),
}
impl Felix{
    pub fn new() -> Self {
        Self{inventory: 0, health_bar: 6, rupee_balance: 0, hand: (None, None), location: (0, 0)}
    }
    pub fn move_felix(&mut self, x: i32, y: i32){ //Keep in mind this function doesn't teleport felix but add's given position
        self.location.0 += x; self.location.1 += y;
    }
    pub fn use_hand_a(&mut self){
        if let Some(item) = self.hand.1.as_mut(){
            item.use_item(self.location, self.rupee_balance);
            if item.is_disposable(){
                if item.count_remove() == 0 {
                    item = None;
                }
            }
        }
    }
    pub fn use_hand_b(&mut self){
        if let Some(item) = self.hand.1.as_mut(){
            item.use_item(self.location, self.rupee_balance);
            if item.is_disposable(){
                if item.count_remove() == 0 {
                    item = None;
                }
            }
        }
    }
}