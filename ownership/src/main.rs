fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_iterger: i32) {
    println!("{}", some_iterger);
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s2);
    println!("{}", s1);

    takes_ownership(s1);
    // println!("s1: {}", s1);
    let x = 5;
    makes_copy(x);
}
