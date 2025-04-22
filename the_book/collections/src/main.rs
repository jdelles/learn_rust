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

    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
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
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        SpreadsheetCell::Float(f) => println!("Float: {}", f),
        SpreadsheetCell::Text(s) => println!("Text: {}", s),
    }

    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("s is {}", s);

    let h1 = String::from("hello, ");
    let h2 = String::from("world");
    let h3 = format!("{}{}", h1, h2); // format! does not take ownership, it just creates a new String
    let h4 = h1 + &h2; // note: `h1` is moved here and can no longer be used
    println!("h3 is {}", h3);

    let hello = String::from("Hello");

    // Bytes

    for b in hello.bytes() {
        println!("{}", b);
    }

    // Scalar values (chars)

    for c in hello.chars() {
        println!("{}", c);
    }

    // Grapheme clusters

    for g in hello.graphemes(true) {
        println!("{}", g);
    }

    // Hashmaps

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(String::from("Blue"), 25); // This will overwrite the value

    scores.insert(yellow, 50);

    println!("scores is {:?}", scores);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("The score for {} is {}", team_name, s),
        None => println!("No score found for {}", team_name),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(30); // This will not overwrite the value

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", map);
}
