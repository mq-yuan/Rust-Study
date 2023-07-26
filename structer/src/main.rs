#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

struct AlwaysEqual;
struct Point(i32, i32, i32); // define by () need ;
                             // struct not allowed to mark only one field as mut
                             // define by {} no ;
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let user3 = User {
        active: false,
        ..user2
    };
    user1.email = String::from("anotheremail@example.com");
    // println!("{}", user2.email); ownership has been moved when create user3

    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };
    // :? only line, :#? multi line;
    println!("the rectangle {:#?} are is {}", rect1, rect1.area());
    // (&rect1).area() == rect1.area();
    dbg!(&rect1); // dbg! will get the ownership so ref is a good idea;
    println!("{:#?}", rect1);
    let rect2 = Rectangle::square(5);
    println!("the rectangle {:#?} are is {}", rect2, rect2.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // &str diff with String
        username,
        email,
        sign_in_count: 1,
    }
}
