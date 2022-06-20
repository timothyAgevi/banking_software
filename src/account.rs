pub struct Account {
    name: String,
    //ID: u8,
    balance: f32
}
impl Account {
    pub fn new(name: &str) -> Self{
        Self { name: name.to_string(), balance: 0.0 }
    }
    //function to get balance
    pub fn getBalance(&self) -> f32{
        self.balance
    }
    //function to reset amou
    pub fn reset_amount(&mut self,amount: f32){
        self.balance=amount;
    }
}