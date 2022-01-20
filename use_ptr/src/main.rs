use std::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // let c = CustomSmartPoint {
    //     data: String::from("my stuff"),
    // };
    // let d = CustomSmartPoint {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created!");
    let p1 = Rc::new(3);
    {
        let p2 = Rc::clone(&p1);
        println!("{}:{}  {}", p1, *p1, Rc::strong_count(&p1));
    }

    println!("{}:{}  {}", p1, *p1, Rc::strong_count(&p1));
}
