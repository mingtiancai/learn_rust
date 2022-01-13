fn another_function() {
    println!("another function")
}

fn another_function2(x: i32) {
    println!("the value is : {} in another function!", x);
}

fn five() -> i32 {
    return 5;
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
    println!("the value of z is: {}", z);
    println!("the value of x is: {}", tup.0);
    println!("the value of y is: {}", tup.1);
    println!("the value of z is: {}", tup.2);
    let a = [1, 2, 3, 4];
    println!("a: {}", a[0]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[4]);
    another_function();
    another_function2(32);
    let f = five();
    println!("f: {}", f);
}
