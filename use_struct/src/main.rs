#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
    println!("{}", user.active);
}

fn main() {
    let user1 = build_user(String::from("as"), String::from("ss"));
    print_user(&user1);
    println!("{:#?}", user1);
    let r = Rectangle {
        width: 30,
        height: 21,
    };
    println!("{:#?}", r);
    println!("area: {}", r.area());
}
