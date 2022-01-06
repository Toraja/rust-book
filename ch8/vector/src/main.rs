fn main() {
    initialise();
    update();
    vec_macro();
    read();
    read_string();
    iterate();
    multiple_type_with_enum();
}

fn initialise() {
    // if no values are added, type annotation is required
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
}

fn update() {
    // type annotation is NOT required as values are added later
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}

fn vec_macro() {
    // macro
    let v3 = vec![3, 4, 5];
    println!("{:?}", v3);
}

fn read() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {} using index", third);

    match v.get(2) {
        Some(third) => println!("The third element is {} using get()", third), // <-
        None => println!("There is no third element."),
    }

    // let hundredth = &v[100]; // <- this will be index out of bounds error
    match v.get(100) {
        Some(hundredth) => println!("The hundredth element is {} using get()", hundredth),
        None => println!("There is no hundredth element."), // <-
    }
}

fn read_string() {
    let v = vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ];
    // let first = v[0]; // <- here it must be borrow as move out of Vector is not allowed
    let first = &v[0];
    println!("{}", first);
}

fn iterate() {
    let mut v = vec![100, 32, 57];
    println!("original: {:?}", v);
    for i in &mut v {
        *i += 50
    }
    println!("mutated:  {:?}", v);
}

fn multiple_type_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
