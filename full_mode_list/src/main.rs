enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum _Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    _liter_value();
    _named_value();
    _multi_modes();
    _seq_modes();
    _deconstruction_struct();
    _deconstruction_enum();
    _deconstruct_nesting();
    _deconstruct_array();
}

fn _liter_value() {
    // If you want code to match the specfic value
    let x = 1;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3 => println!("x is 3"),
        _ => println!("anything"),
    };
}

fn _named_value() {
    // y, which in Some(y), will bind to the matching value
    let x = Some(50);
    let y = 10;
    match x {
        Some(5) => println!("x is 5"),
        Some(y) => println!("x is {}", y),
        _ => println!("Anything"),
    };
    println!("x, y is {:?}, {}", x, y);
}

fn _multi_modes() {
    let x = 10;
    match x {
        2 | 10 => println!("x is 2 or 10"),
        15 => println!("x is 15"),
        _ => println!("anything"),
    };
}

fn _seq_modes() {
    // seq only support number and char
    // Because number and cahr is continuous
    let x = 3;
    match x {
        1..=10 => println!("x in 1~10"),
        _ => println!("Anything"),
    };
    let x = 'c';
    match x {
        'a'..='z' => println!("x in a~z"),
        _ => println!("Anything"),
    };
}

fn _deconstruction_struct() {
    let p = Point { x: 2, y: 5 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    let ((_feet, _inches), Point { x: _x, y: _y }) = ((3, 10), Point { x: 2, y: 5 });
    assert_eq!(2, a);
    assert_eq!(5, b);
    assert_eq!(2, x);
    assert_eq!(5, y);
    println!("p is {:?}", p);

    match p {
        Point { x, y: 0 } => println!("p is Point({}, 0)", x),
        Point { x: 0, y } => println!("p is Point(0, {})", y),
        Point { x, y } => println!("p is Point({}, {})", x, y),
    };
}

fn _deconstruction_enum() {
    let msg = _Messages::ChangeColor(0, 5, 255);

    match msg {
        _Messages::Quit => println!("The Quit variant has no data to destructure"),
        _Messages::Move { x, y } => println!("Move is (x:{}, y:{})", x, y),
        _Messages::Write(text) => println!("Text message is {}", text),
        _Messages::ChangeColor(r, g, b) => println!("Color is r:{}, g:{}, b:{}", r, g, b),
    };
}

fn _deconstruct_nesting() {
    let msg = Messages::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Messages::Quit => println!("The Quit variant has no data to destructure"),
        Messages::Move { x, y } => println!("Move is (x:{}, y:{})", x, y),
        Messages::Write(text) => println!("Text message is {}", text),
        Messages::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Color is r:{}, g:{}, b:{}", r, g, b);
        }
        Messages::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Color is h:{}, s:{}, v:{}", h, s, v);
        }
    };
}

fn _deconstruct_array() {
    let array: [u16; 2] = [12, 15];
    let [x, y] = array;
    assert_eq!(x, 12);
    assert_eq!(y, 15);

    let array: &[u16] = &[114, 115, 116];
    if let [x, ..] = array {
        assert_eq!(x, &114);
    }
    if let [.., y] = array {
        assert_eq!(y, &116);
    }
    let arr: &[u16] = &[];
    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [_x, ..]));
}
