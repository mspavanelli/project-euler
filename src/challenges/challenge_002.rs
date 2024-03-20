pub fn main() {
    const LIMIT: i32 = 4 * i32::pow(10, 6);

    let mut sum: i32 = 0;
    let mut i = 0;
    loop {
        i += 1;
        let fib = fibonacci(i);
        if fib > LIMIT {
            break;
        };
        if fib % 2 == 0 {
            sum += fib;
        }
    }

    println!("002: {}", sum);
}

pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    };
    return fibonacci(n - 1) + fibonacci(n - 2);
}
