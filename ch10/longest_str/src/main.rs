fn main() {
    same_lifetime();
    shorter_is_valid();
    shorter_is_valid_another();
    longest_struct();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn same_lifetime() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let longest = longest(s1.as_str(), s2);
    println!("The longest string is {}", longest);
}

fn shorter_is_valid() {
    let s1 = String::from("long string is long");

    {
        let s2 = String::from("xyz");
        let longest = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {}", longest);
    }
}

#[allow(dead_code)]
fn shorter_is_invalid() {
    // XXX This does not compile
    // let s1 = String::from("long string is long and long");
    // let result;

    // {
    //     let s2 = String::from("this dies before printed");
    //     result = longest(s1.as_str(), s2.as_str());
    // }
    // println!("The longest string is {}", result);
}

fn shorter_is_valid_another() {
    let s1 = String::from("long string is long and long");
    let result;
    let s2;

    {
        s2 = String::from("this will be NOT dead when printed");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("The longest string is {}", result);
}

struct Longer<'a> {
    x: &'a str,
    y: &'a str,
}

impl Longer<'_> {
    fn get(&self) -> &str {
        if self.x.len() > self.y.len() {
            self.x
        } else {
            self.y
        }
    }
}

fn longest_struct() {
    let s1 = String::from("longest");
    let s2 = "short";
    let longer = Longer {
        x: s1.as_str(),
        y: s2,
    };
    println!("{}", longer.get());

    // NOTE The below does not compile as s4 lives shorter than Longer struct
    // let s3 = String::from("longest");
    // let longer;
    // {
    //     let s4 = String::from("this is not short");
    //     longer = Longer {
    //         x: s3.as_str(),
    //         y: s4.as_str(),
    //     };
    // }
    // println!("{}", longer.get()); // reference to s4
}
