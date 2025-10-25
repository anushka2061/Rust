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
    let mut count=0;
	if let Coins::Quarter(state) =coin{
		println!("State quarter from {state:?}!");
	}
	else{
	count+=1;
	}
    
}

fn main(){
	let result = value_in_cents(Coins::Quarter(Alaska));
	println!("The result is {result}");
	}
