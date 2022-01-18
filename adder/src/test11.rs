#[cfg(test)]
pub mod test11 {
    use crate::lib::*;
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contain_name() {
        let result = greeting("Cargo");
        assert!(
            result.contains("Cargo"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn iii() -> Result<(), String> {
        if 2 + 2 == 3 {
            Ok(())
        } else {
            Err(String::from("two plus two does not euqal four"))
        }
    }
}
