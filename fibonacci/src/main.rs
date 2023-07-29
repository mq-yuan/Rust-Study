fn main() {
    let mut fibonacci: [i32; 2] = [1, 1];
    let mut inum = String::new();

    std::io::stdin()
        .read_line(&mut inum)
        .expect("Failed to read");
    let num: usize = inum.trim().parse().expect("Failed to parse number {inum}");

    if num <= 2 {
        let result = fibonacci[num - 1];
        println!("The result is {result}");
        return;
    }
    for i in 2..num {
        fibonacci[i % 2] = fibonacci[0] + fibonacci[1];
    }
    let result = fibonacci[(num - 1) % 2];
    println!("The result is {result}");
}
