pub fn main() {
    println!("003: {}", largest_prime_factor(600851475143));
}

pub fn largest_prime_factor(number: i64) -> i64 {
    let mut largest_prime = -1;
    let mut i = 2;
    let mut n = number;

    while i * i <= n {
        while n % i == 0 {
            largest_prime = i;
            n /= i;
        }
        i += 1;
        if n > 1 {
            largest_prime = n
        }
    }
    largest_prime
}
