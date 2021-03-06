fn use_while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn use_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color ,{}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn use_match() {
    let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched,y = {:?}", y),
        _ => println!("Default case , x={:?} , y={:?}", x, y),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

fn use_match2() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn main() {
    // use_if_let();
    // use_while_let();
    use_match2();
}
