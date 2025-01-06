fn main() {
    let n = 5;
    let mut memo = [-1; 100];
    let nth_fibonacci = fibonacci(n, &mut memo);
    println!("{}", nth_fibonacci); // sequence is 0, 1, 1, 2, 3, etc.... 3 is the 5th value
}

fn fibonacci(n: usize, memo: &mut [i32]) -> i32 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if memo[n] != -1 {
        return memo[n];
    }
    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo[n] = result;
    result
}