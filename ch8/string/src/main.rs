use unicode_segmentation::UnicodeSegmentation;

fn main() {
    initialise();
    string_slice();
    update_string();
    concatenate();
    slicing_string();
    iterate_string();
}

fn initialise() {
    let s = String::new();
    println!("{}", s);
}

fn string_slice() {
    fn print_type_of<T>(_: &T, msg: &str) {
        println!("{} -> {}", msg, std::any::type_name::<T>())
    }

    let data = "initial contents";
    print_type_of(&data, "data");

    // these all do the same thing
    let _s = data.to_string();
    print_type_of(&_s, "_s");
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");
}

fn update_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let mut l = String::from("lo");
    l.push('l');
    println!("l is {}", l);
}

fn concatenate() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // <- This is actually calling String.add(self, &str)
    println!("s1 has been moved, s2: {}, s3: {}", s2, s3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let f = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", f);
}

fn slicing_string() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);
    // println!("{}", &hello[0..5]); // <- this panics
}

fn iterate_string() {
    let s = String::from("नमस्ते");
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }
    for g in s.graphemes(true).collect::<Vec<&str>>() {
        println!("{}", g);
    }
}
