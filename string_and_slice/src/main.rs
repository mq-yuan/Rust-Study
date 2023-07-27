fn main() {
    _string_operate();
    _string_escape();
    _unicode_operate();
}

fn _string_operate() {
    // push: (push_str, push)
    let mut s = String::from("Hello ");
    s.push_str("rust");
    s.push('!');
    dbg!(s);

    // insert: (insert, insert_str)
    s = String::from("Hello rust!");
    s.insert(5, ',');
    s.insert_str(6, " I like");
    dbg!(s);

    // replace: (replace, replacen, replace_range)
    let string_replace = String::from("I like rust. Learning rust is my favorite");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    let new_string_replace = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replace);
    let mut string_replace_range = string_replace;
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // delete: (pop, remove, truncate, clear)
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    let mut string_remove = String::from("测试remvoe方法");
    println!(
        "string_remove has {} bytes",
        std::mem::size_of_val(string_remove.as_str())
    );

    // string_remove.remove(1); slice index by bytes, so 1 is part of "测"
    string_remove.remove(0);
    dbg!(string_remove);
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // concatenate: (+, +=, format!)
    // fn add(self, s: &str) -> String
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    dbg!(result);
    dbg!(string_rust);
    // dbg!(string_append): 54 ownership to result
    let s1 = "Hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    dbg!(s);
}

fn _string_escape() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called{}",
        unicode_codepoint, character_name
    );
    let long_string = "String
        can span multiple lines.
                The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
    let raw_str = r"Escape don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    let quotes = r#" This " is no escape!"#;
    println!("{}", quotes);
    let longer_delimiter = r###" "# in it, and even "##!"###;
    println!("{}", longer_delimiter);
}

fn _unicode_operate() {
    println!("这里是china");
    for c in "这里是china".chars() {
        println!("{}", c);
    }
    println!("这不好wuwu");
    for b in "这不好wuwu".bytes() {
        println!("{}", b);
    }
}
