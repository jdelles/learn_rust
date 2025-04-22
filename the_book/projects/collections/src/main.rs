use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1, 2, 3];
    }

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a interger!"),
    };

    // Strings are stored as a collection of utf-8 bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4: String = String::from("initial contents");

    // foobar
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    // Hello, world!
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6;
    let s8 = format!("{}{}", s1, s2);

    let hello: String = String::from("Hello");
    // let c: char = hello[0]; <-- doesn't work

    for b in hello.bytes() {
        println!("{}", b);
    }

    for char in hello.chars() {
        println!("{}", char);
    }

    for g in hello.graphemes(true) {
        println!("{}", g);
    }

    // Hashmaps
    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name: String = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    let text: &str = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
