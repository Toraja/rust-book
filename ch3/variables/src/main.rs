fn main() {
    println!("--- mutablity ---");
    mutablity();
    println!("--- shadowing ---");
    shadowing();
}

fn mutablity() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;

    println!("The value of x before shadowing is: {}", x);

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
