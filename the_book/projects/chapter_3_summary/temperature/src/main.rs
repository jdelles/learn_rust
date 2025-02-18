fn main() {
    let c = 0.0;
    let f = celsius_to_fahrenheit(c);
    println!("{f}");

    let f = 32.0;
    let c = fahrenheit_to_celsius(f);
    println!("{c}");
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
