enum Dirrection {
    East,
    West,
    North,
    South,
}

enum IpAddr {
    V4,
    V6,
}

#[derive(Debug)]
enum UsState {
    Alabma,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabma);
    _math();
    value_in_cents(coin);
    if_let();
    _matches();
    _variable_masking();
    _while_let();
}

fn _math() -> u8 {
    let dir = Dirrection::East;

    let ip1 = IpAddr::V6;
    let ip2 = match ip1 {
        IpAddr::V4 => "127.0.0.1",
        IpAddr::V6 => "::1",
    };

    println!("{}", ip2);

    // Notice if match return a value, need not to use ;
    match dir {
        Dirrection::East => {
            println!("East");
            1
        }
        Dirrection::West | Dirrection::North => {
            println!("West or North");
            2
        }
        _ => 1,
    } // here NO ;
}

fn value_in_cents(coin: Coin) -> u8 {
    let _state = UsState::Alaska;
    match _state {
        UsState::Alabma => println!("Alabma"),
        other => println!("{:?}", other),
    };
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?} ", state);
            25
        }
    }
}

fn if_let() {
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    };
}

fn _matches() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // BAD v.iter().filter(|x| x == MyEnum::Foo);
    let _ = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    let foo = 'f';
    let bar = Some(4);
    assert!(matches!(foo, 'A'..='Z'|'a'..='z'));
    assert!(matches!(bar, Some(x) if x > 2));
}

fn _variable_masking() {
    // BAD Use
    let age = Some(3);
    println!("{:?}", age);
    if let Some(age) = age {
        println!("{}", age);
    }
    println!("{:?}", age);
}

fn _while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(p) = stack.pop() {
        println!("{}", p);
    }
}
