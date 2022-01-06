use std::collections::HashMap;

fn main() {
    basic();
    collect();
    ownership();
    iterate();
    update();
}

fn basic() {
    let _hm: HashMap<String, i32> = HashMap::new();

    let scores = default_map();
    println!("{:?}", scores);

    let team_name = String::from("Blue"); // need to redeclare "Blue" here as `blue` has been moved
    let score = scores.get(&team_name);
    println!("score for team {} is {}", team_name, score.unwrap());

    let team_name = String::from("Green");
    match scores.get(&team_name) {
        Some(score) => println!("score for team {} is {}", team_name, score),
        None => println!("Team {} does not exist", team_name),
    }
}

fn collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50, 70]; // <- extra elements (70) is ignored

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
}

fn ownership() {
    let key = String::from("Favorite color");
    let value = String::from("Green");
    let mut map = HashMap::new();
    map.insert(key, value);
    println!("{:?}", map);
    // The below does not compile as insert() method takes ownership
    // println!("key: {}, value: {}", key, value);
}

fn iterate() {
    let scores = default_map();
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    for (key, value) in scores {
        // `scores` is moved here
        println!("{}: {}", key, value);
    }
    // println!("{:?}", scores); // does not compile
}

fn update() {
    // overwrite
    let mut scores = default_map();
    scores.insert(String::from("Blue"), 99);
    println!("{:?}", scores);

    // insert if not exist
    let mut scores = default_map();
    scores.entry(String::from("Blue")).or_insert(88);
    scores.entry(String::from("Green")).or_insert(22);
    println!("{:?}", scores);

    // update based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn default_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores
}
