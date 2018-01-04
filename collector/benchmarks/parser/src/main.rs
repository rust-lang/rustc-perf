extern crate combine;

mod parser;

pub type Num = i64;

pub struct Stats {
    pub capacity: Num,
    pub durability: Num,
    pub flavor: Num,
    pub texture: Num,
    pub calories: Num,
}
impl Stats {
    fn zero() -> Stats {
        Stats { capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 }
    }
}

fn main() {
    println!("Hello, world!");
}
