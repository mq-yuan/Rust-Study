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

fn main() {
    let coin = Coin::Quarter(UsState::Alabma);
    _math();
    value_in_cents(coin);
    if_let();
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
