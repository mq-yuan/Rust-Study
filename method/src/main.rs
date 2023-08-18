struct Circle {
    p: (f64, f64),
    r: f64,
}

impl Circle {
    pub fn new(p: (f64, f64), r: f64) -> Circle {
        Circle { p, r }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r * self.r)
    }
}

impl Circle {
    pub fn p(&self) -> (f64, f64) {
        self.p
    }

    pub fn r(&self) -> f64 {
        self.r
    }
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("This is a method for Message");
    }
}

fn main() {
    let circle1 = Circle::new((0.0, 0.0), 2.0);
    println!(
        "The Circle({:?}, {}) area is {}",
        circle1.p(),
        circle1.r(),
        circle1.area()
    );
    let message = Message::Write(String::from("GOOD"));
    message.call();
}
