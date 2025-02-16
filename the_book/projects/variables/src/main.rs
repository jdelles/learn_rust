fn main() {
    let mut x = 5;
    println!("The values of x is: {}", x);
    x = 6;
    println!("The values of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The values of x is {x}");

    // scalar types
    // Integer

    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)
    
    
    // Floating-point numbers

    let f: f32 = 2.0; // f32
    let g: f64 = 3.0; // f64

    // Numeric operations

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Booleans

    let t: bool = true;
    let f: bool = false;

    // Characters

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound types

    // Tuple

    let tup: (&str, i32) = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found: i32 = error_codes[2];

    let byte = [0; 8]; 
}
