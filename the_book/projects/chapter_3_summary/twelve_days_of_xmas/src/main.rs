fn main() {
    const VERSES: [&str; 12] = [
        "A partridge in a pear tree.",
        "Two turtle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a laying,",
        "Seven swans a swimming,",
        "Eight maids a milking,",
        "Nine ladies dancing,",
        "Ten lords a leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in 1..=12 {
        println!(
            "On the {} day of Christmas my true love gave to me:",
            ordinal(day)
        );
        for verse in (0..day).rev() {
            println!("{}", VERSES[verse]);
        }
    }
}

fn ordinal(n: usize) -> &'static str {
    match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => unreachable!(),
    }
}
