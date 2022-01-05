fn main() {
    let coins: [Coin; 5] = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ];
    for coin in coins {
        println!("Value of {:?} is {}", coin, value_in_cents(&coin));
        if let Coin::Quarter(state) = coin {
            println!("Have you been to {:?}?", state);
        } else if let Coin::Penny = coin {
            println!("That's not much");
        } else if let Coin::Nickel = coin {
            println!("Better but still...");
        }
        println!("{}", "-".repeat(30));
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
