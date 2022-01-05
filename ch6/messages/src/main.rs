fn main() {
    let m = Message::Quit;
    m.print();
    let m = Message::Move { x: 3, y: 4 };
    m.print();
    let m = Message::Write(String::from("hello"));
    m.print();
    m.call();
    let m = Message::ChangeColor(5, 6, 7);
    m.print();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}

    fn print(&self) {
        println!("{:?}", &self);
    }
}
