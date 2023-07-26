fn main() {
    // CONST
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("const is {THREE_HOURS_IN_SECONDS}");

    // variables
    let mut x: u32 = 6;
    let mut y = 7; // i32
    println!("x is {x}, y is {y}");
    x = x + 1;
    y = y + 1;
    println!("x is {x}, y is {y}");
    let z: u32 = 16;
    println!("z is {z}");
    {
        println!("{{");
        // z = z + 1 is Error
        let z = 21;
        println!("z is {z}");
        x = x * 10;
        println!("x is {x}, y is {y}");
        println!("}}");
    }
    println!("x is {x}, y is {y}");
    println!("z is {z}");
}
