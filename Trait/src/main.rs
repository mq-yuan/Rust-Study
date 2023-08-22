use std::{
    fmt::{Debug, Display},
    ops::Add,
};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// impl<T: Display> ToString for T {
//     // snip
// }

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is {}", self.x);
        } else {
            println!("The largest number is {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Something by {}", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!(
            "The Post Title is {}, and the author is {}",
            self.title, self.author
        )
    }
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        self.username.clone()
    }
}

pub fn some_func<T, U>(_item1: &T, _item2: &U)
where
    T: Display + Clone,
    U: Display + Clone + Debug,
{
}

// pub fn notify(item: &impl Summary) {
//     println!("!!!!BREAKING NEWS!!!! \"{}\"", item.summarize());
// }
// pub fn notify<T: Summary + Display>(item: &T);
pub fn notify<T: Summary>(item: &T) {
    println!("!!!!BREAKING NEWS!!!! \"{}\"", item.summarize());
}

// only return the same type
fn returns_summarizale() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 max太厉害了，电脑再也不会卡"),
    }
}

fn main() {
    let post = Post {
        title: "Rust".to_string(),
        author: "YMQ".to_string(),
        content: "Rust is best".to_string(),
    };
    let weibo = returns_summarizale();
    let pair = Pair::new(2.2, 32.7);
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    notify(&post);
    pair.cmp_display();
    let point1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let point2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(point1, point2));
    let point1 = Point { x: 1i32, y: 1i32 };
    let point2 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(point1, point2));
}

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}
