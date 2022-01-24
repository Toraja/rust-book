fn main() {
    func::rectangle();

    separate();

    tuple::rectangle();

    separate();

    structed::rectangle();

    separate();

    method::rectangle();
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

mod method {
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // static method can be defined like normal method but without `self` as the first
        // parameter.
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    pub fn rectangle() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels",
            rect1.area(),
        );

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        println!("Square: {:#?}", Rectangle::square(5));
    }
}

#[cfg(test)]
mod tests {
    use super::method::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(!smaller.can_hold(&larger));
    }
}
