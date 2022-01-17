fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut l = list[0];
    for &item in list.iter() {
        if item > l {
            l = item;
        }
    }
    l
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more ....)")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'x' };
    let p3 = p1.mixup(p2);
    println!("p3.x={},p3.y={}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,as your probably already know,people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let number_list = vec![32, 33, 34, 45];
    let result = largest(&number_list);
    println!("largest number is : {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("the largest number is : {}", result);

    let string1 = String::from("long string");
    let result4;

    let string2 = String::from("string");
    result4 = longest(string1.as_str(), string2.as_str());
    println!("the longest string is {}", result4);
}
