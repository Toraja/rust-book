use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let dice_roll = random_num(12);
    println!("You've got {}", dice_roll);
    match dice_roll {
        2 | 11 => reroll(),
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        9 => (), // nothing happens
        other => forward(other),
    }
}

fn add_fancy_hat() {
    println!("Congrats! You've won a fancy hat!");
}
fn remove_fancy_hat() {
    println!("Sorry, we have to take your hat.");
}
fn forward(num_spaces: u32) {
    println!("Forward {} spaces", num_spaces);
}
fn reroll() {
    println!("Roll the dice again");
}

fn random_num(max: u32) -> u32 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get duration")
        .as_millis()
        % max as u128) as u32
        + 1
}
