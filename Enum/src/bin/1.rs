#[derive(Debug)]
enum USstate {
    Alaska,
    California,
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(USstate),
}

fn value_in_cents(coin: Coins) -> u32 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coins::Quarter(USstate::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));
}
