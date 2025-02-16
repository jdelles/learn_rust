fn main() {
    /*
        Ownership Rules: 
            1. Each value in Rust has a variable that’s called its owner.
            2. There can only be one owner at a time.
            3. When the owner goes out of scope, the value will be dropped.
     */

     let s = String::from("hello"); // s is valid from this point forward

     let x = 5;
     let y = x; // copy

    let s1 = String::from("hello");
    // let s2 = s1; // move  (s1 no longer valid)
    let s2 = s1.clone(); // copy

    println!("{}, world!", s1);

    takes_ownership(s);
    // println!("{}", s); // invalid

    let s3 = gives_ownership();
    println!("{}", s3);

    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    println!("{}", s5);

    let len = calculate_length(&s5);
    println!("The length of '{}' is {}", s5, len);

    let mut s7 = String::from("hello");
    change(&mut s7);
    println!("{}", s7);

    let mut s8 = String::from("hello world");
    let hello = &s8[0..5];
    let world = &s8[6..11];
    let word = first_word(&s8);
    s8.clear();

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // invalid

/*
    The Rules of References:
        1. At any given time, you can have either one mutable reference or any number of immutable references.
        2. References must always be valid.
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}