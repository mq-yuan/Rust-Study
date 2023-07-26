use num::complex::Complex;
use std::ops::{Range, RangeInclusive};

fn main() {
    println!("Hello, world!");
    integer_overflow();
    float_trap();
    nan();
    digital_operate();
    bit_operate();
    range();
    println!();
}

fn integer_overflow() {
    // wrapping_*: two's complement wrapping
    // checked_*: overflow->None
    // overflowing_*: return value(wrapping value) and bool(for overflow)
    // saturating_*: return sturct max or min
    println!("\ninteger_overflow");
    let _c: u16 = 38_u8 as u16;
    let a: u8 = 255;
    let b = a.saturating_add(13);
    println!("{}", b);
}

fn float_trap() {
    // float is approximate
    // float always are counterintuitive in some characteristics(float only PartialEq not Eq)
    // AVOID testing equal on float
    println!("\nfloat_tarp");
    println!("{}", 0.1 + 0.2 == 0.3);

    // a metod to cmp, you can set precision
    println!("{}", (0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    // a complex example
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!("abc.0 + abc.1 == abc.2 is {}", abc.0 + abc.1 == abc.2);
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!("xyz.0 + xyz.1 == xyz.2 is {}", xyz.0 + xyz.1 == xyz.2);
    println!();
}

fn nan() {
    // NaN(not a number) is float
    // All operations that interact with NaN will return NaN
    // NaN can not be used for cmp
    println!("\nnan");
    let x = (-42_f32).sqrt();
    if x.is_nan() {
        println!("-42.sqrt() is Undefine");
    }
}

fn digital_operate() {
    println!("\ndigital_operate");
    // add
    let _sum = 5 + 10;
    // difference
    let _difference = 95.3 - 4.2;
    // product
    let _product = 4 * 10;
    // quotient
    let _quotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;

    // comprehensive example
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    // only the same type can be operated
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_two, twenty_one, addition
    );

    // yout can use "_" to segment digital to imporve readability
    let one_million: i64 = 1_000_000;
    println!("{}", one_million);

    // array inference
    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:.2}", forty_twos[0]);
}

fn bit_operate() {
    println!("\nbit_operate");
    // &, |, ^, !, <<, >>(symbol)
    let a: i32 = 2;
    let b: i32 = 3;
    println!("a is {}, b is {}", a, b);
    println!("(a & b) is {}", a & b);
    println!("(a | b) is {}", a | b);
    println!("(a ^ b) is {}", a ^ b);
    println!("(!b) is {}", !b);
    println!("(a << b) is {}", a << b);
    println!("(a >> b) is {}", a >> b);

    let mut a = a;
    a <<= b;
    println!("(a << b) is {}", a);
}

fn range() {
    println!("\nrange");
    // range only allowed number and char, because they are continuous
    // 1..4 is (1, 2, 3), 1..=4 is (1, 2, 3, 4)
    // 1..4 == (Range{ start: 1, end: 5})
    // 1..=4 == RangeInclusive::new(1, 5)
    for i in (Range { start: 1, end: 5 }) {
        println!("{}", i);
    }
    for i in RangeInclusive::new('a', 'd') {
        println!("{}", i);
    }
}

fn _complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
