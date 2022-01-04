fn main() {
    func::rectangle();

    separate();

    tuple::rectangle();

    separate();

    structed::rectangle();
}

fn separate() {
    println!("{}", "-".repeat(50));
}

mod func {
    pub fn rectangle() {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels",
            area(width1, height1)
        );
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

mod tuple {
    pub fn rectangle() {
        let rect = (30, 50);

        println!("The area of the rectangle is {} square pixels", area(rect));
    }

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

mod structed {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    pub fn rectangle() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels",
            area(&rect1)
        );
        println!("rect1 is {:#?}", rect1);

        let scale = 2;
        let rect2 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        dbg!(&rect2);
    }

    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}
