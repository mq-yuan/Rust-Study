fn main() {
    _string_operate();
}

fn _string_operate() {
    let mut s = String::from("Hello ");
    s.push_str("rust");
    s.push('!');
    println!("{}", s);
}
