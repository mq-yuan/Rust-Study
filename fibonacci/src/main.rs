fn main() {
    let num = read_num();

    println!("The result_my is {}", fibonacci_me(num));
    println!("The result codegeex is {}", fibonacci_codegeex(num));
}

fn fibonacci_codegeex(num: usize) -> usize {
    if num <= 2 {
        return 1;
    }
    let mut fibonacci: [usize; 2] = [1, 1];
    for i in 2..num {
        fibonacci[i % 2] = fibonacci[0] + fibonacci[1];
    }
    fibonacci[(num - 1) % 2]
}

// 帮我完成从命令行中读入数字的函数，并取一个恰当的名字
fn read_num() -> usize {
    let mut inum = String::new();
    std::io::stdin()
        .read_line(&mut inum)
        .expect("Failed to read");
    inum.trim().parse().expect("Faile")
}

fn fibonacci_me(num: usize) -> usize {
    let mut fibonacci: [usize; 2] = [1, 1];
    if num <= 2 {
        return fibonacci[num - 1];
    }
    for i in 2..num {
        fibonacci[i % 2] = fibonacci[0] + fibonacci[1];
    }
    fibonacci[(num - 1) % 2]
}
