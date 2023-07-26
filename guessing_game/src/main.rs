use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gusses the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your Number:");
        let mut gusses = String::new();
        io::stdin()
            .read_line(&mut gusses)
            .expect("Failed to read line");

        let gusses: u32 = match gusses.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match gusses.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
