use std::fmt;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speeking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

struct Warpper(Vec<String>);

impl fmt::Display for Warpper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn use_func_ptr() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

#[macro_export]
macro_rules! vec2 {
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
};
}

fn main() {
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    let w = Warpper(vec![String::from("hello"), String::from("world")]);
    println!("w={}", w);
    use_func_ptr();
    let v = vec2![9, 2, 3];

    for i in v {
        println!("{}", i);
    }
}
