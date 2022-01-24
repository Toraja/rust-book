// some tests are added to CH2 and CH5
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail")
    }

    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not ccontain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 3 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }
}
