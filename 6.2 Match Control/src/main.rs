#[derive(Debug)]
enum UserState {
    Alabama,
    Alaska,
    // Add more states as needed
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UserState),
}
fn main() {
    println!("Hello, world!");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}