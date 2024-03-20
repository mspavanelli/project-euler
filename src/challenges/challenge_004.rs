pub fn main() {
    println!("004: {}", largest_palindrome_product());
}

pub fn largest_palindrome_product() -> i32 {
    let mut largest: i32 = -1;

    for first in 100..1000 {
        for second in 100..1000 {
            let product = first * second;
            if is_palindromic_number(product) && product > largest {
                largest = product
            }
        }
    }
    largest
}

pub fn is_palindromic_number(number: i32) -> bool {
    let number_string = number.to_string();
    let reversed_string: String = number_string.chars().rev().collect();
    number_string == reversed_string
}

#[cfg(test)]
mod tests {
    use crate::challenges::challenge_004::is_palindromic_number;

    #[test]
    fn is_palindromic() {
        let test_cases = [0, 1, 11, 22, 101, 121, 999, 9009];
        for number in test_cases {
            assert_eq!(is_palindromic_number(number), true);
        }
    }

    #[test]
    fn is_not_palindromic() {
        let test_cases = [10, 12, 100, 990, 9909];
        for number in test_cases {
            assert_eq!(is_palindromic_number(number), false);
        }
    }
}
