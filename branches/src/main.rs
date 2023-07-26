fn main() {
    // if
    let x: u32 = 3;
    let y: u32 = if 3 == 0 { 1 } else { 0 };
    println!("x is {x}, y is {y}");

    // loop only break
    let mut condition: i32 = 1;
    let result: i32 = loop {
        condition += 1;
        if condition == 10 {
            break condition * 2;
        }
    };
    println!("condition is {condition}, result is {result}");

    // loop label
    'count_up: loop {
        let mut remaining = 10;
        loop {
            println!("remaining is {remaining}");
            if remaining == 9 {
                break;
            }
            if condition == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }
        condition -= 1;
    }
    println!("End condition is {condition}");

    // loop for
    for number in (1..4).rev() {
        println!("{number}");
    }

    // loop while
    while condition != 5 {
        condition += 1;
        println!("condition is {condition}");
    }
}
