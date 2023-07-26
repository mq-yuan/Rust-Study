fn main() {
    let s_1 = "hello";
    let s_2 = String::from("hello");
    println!("{}", s_1);
    println!("{}", s_2);
    let s_3 = s_2;
    // println!("{}", s_2); the ownership has to s_3
    println!("{}", s_3);
    take_ownership(s_3);
    // println!("{}", s_3); the ownership in take_ownership

    let s_4 = String::from("hello");
    let r_1 = &s_4;
    let r_2 = &s_4;
    let r_3 = &s_4;
    let s_5 = dangle();
    println!("{}{}{}{}", r_1, r_2, r_3, s_5);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[1..len]; // slice is the ref of s
    let f = first_world(&slice); // f is the mut ref of slice and s
                                 // let mut a = &mut s; f have the mut ref
                                 // s.clear(); f have the mut ref and clear need mut ref
    println!("{}", f);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
