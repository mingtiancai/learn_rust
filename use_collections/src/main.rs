use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{}", v[0]);
    println!("{}", v[1]);

    let p = v.get(0);
    match p {
        Some(p) => println!("{}", p),
        None => println!("None"),
    }

    for i in &v {
        println!("{}", i);
    }

    let name = "高明";
    for c in name.chars() {
        println!("{}", c);
    }

    let mut secores = HashMap::new();
    secores.insert(String::from("blue"), 10);
    // secores.insert(String::from("red"), 20);

    secores.entry(String::from("red")).or_insert(20);
    secores.entry(String::from("blue")).or_insert(20);

    for i in secores {
        println!("{}: {}", i.0, i.1);
    }
}
