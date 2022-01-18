mod test11;

pub mod lib {
    #[derive(Debug)]
    pub struct Rectangle {
        pub length: u32,
        pub width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }

    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100,got {}.", value);
            }
            Guess { value }
        }
    }
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
}

#[cfg(test)]
mod tests{
    use super::lib::*;
    #[test]
    fn tt(){
        assert_eq!(4,add_two(2));
    }
}