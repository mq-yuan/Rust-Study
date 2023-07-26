fn main() {
    string_first_meet();
    _copy();
    _move();
    _deep_clone();
    _func_pass_and_ret();
}

fn string_first_meet() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn _copy() {
    // Copy:
    //      - basic type
    //      - types that do not require memory allocation or other resource
    // example:
    //      - int: u32 e.t.
    //      - bool: true and false
    //      - float: f64 e.t.
    //      - char: ''
    //      - tuple: only included type that can be Copy, (u32, u32)
    //      - Immutable ref: &T. BUT &mut T is bad
    let x = 5;
    let y = x;
    assert_eq!(x, y);
    let x: &str = "hello, world";
    let y = x;
    assert_eq!(x, y);
}

fn _deep_clone() {
    let s1 = String::from("deep clone");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn _move() {
    let s1 = String::from("move");
    let s2 = s1;
    eprintln!("s1 value used here after move");
    println!("s2 is {}", s2);
}

fn _func_pass_and_ret() {
    let s = String::from("hello");
    let x = 5;
    _takes_ownership(s);
    println!("s has been moved");
    makes_copy(x);
    println!("x still can be used");

    let _s1 = _gives_ownership();
    let _s2 = String::from("hello");
    let _s3 = _takes_and_gives_back(_s2); // s2 move to func, and func ret to s3
}

fn _takes_ownership(some_string: String) {
    // some_string move into func
    println!("{}", some_string);
    // some_string been drop
}

fn makes_copy(some_integer: i32) {
    // some_integer copy into func
    println!("{}", some_integer);
}

fn _gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_string move out func
}

fn _takes_and_gives_back(a_string: String) -> String {
    a_string
}
