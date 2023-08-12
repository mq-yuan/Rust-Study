fn main() {
    branch_control();
    loop_control();
    multilayer_loop();
}

fn branch_control() {
    let condintion = true;
    let number = if condintion { 5 } else { 6 };
    println!("The value of number is: {}", number);

    if number < 5 {
        println!("condition was false");
    } else if number == 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn loop_control() {
    // for i in collection <=> for i in IntoInterator::into_iter(collection)-> ownership transfer
    // for i in &collection <=> for i in collection.iter() -> Unvarivable borrow
    // for i in &mut collection <=> for i in collection.iter_mut()-> varivable borrow
    for i in 1..=5 {
        println!("{}", i);
    }
    let a = [4, 3, 2, 1, 0];
    for (i, v) in a.iter().enumerate() {
        if i == 2 {
            continue;
        } else if i == 5 {
            break;
        }
        println!("The {} number is {}", i, v);
    }

    // while
    let mut n = 0;
    while n <= 5 {
        n = n + 1;
    }
    println!("6 == {}", n);

    n = 0;
    loop {
        if n > 5 {
            break;
        }
        n = n + 1;
    }
    println!("6 == {}", n);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("counter is {}", result);
}

fn multilayer_loop() {
    let mut counter = 0;
    'outer: loop {
        'inter1: loop {
            if counter >= 20 {
                break 'inter1;
            }
            counter += 20;
        }

        counter += 5;

        'inter2: loop {
            if counter >= 30 {
                break 'outer;
            }
            break 'inter2; // <=> break <=> continue 'outer
        }
    }
    println!("counter is {}", counter);
}
