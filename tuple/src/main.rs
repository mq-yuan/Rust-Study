fn main() {
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;
    let _five_hundred = _tup.0;
    let s1 = String::from("hello");
    let (_s2, _len) = calculate_legnth(s1);
}

fn calculate_legnth(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
