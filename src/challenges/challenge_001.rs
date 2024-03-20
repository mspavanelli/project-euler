pub fn main() {
    println!("001: {}", sum_of_all_multiple_of_three_or_five_below(1000));
}

pub fn sum_of_all_multiple_of_three_or_five_below(value: i32) -> i32 {
    let mut sum = 0;
    for number in 1..value {
        if is_multiple_of_three_or_five(number) {
            sum += number
        }
    }
    sum
}

pub fn is_multiple_of_three_or_five(value: i32) -> bool {
    return value % 3 == 0 || value % 5 == 0;
}

#[cfg(test)]
mod tests {
    use crate::challenges::challenge_001::{
        is_multiple_of_three_or_five, sum_of_all_multiple_of_three_or_five_below,
    };

    #[test]
    fn should_be_divisible_by_three_or_five() {
        let test_cases = [0, 3, 5, 6, 9, 10];

        for number in test_cases {
            assert_eq!(is_multiple_of_three_or_five(number), true);
        }
    }

    #[test]
    fn should_not_be_divisible_by_three_or_five() {
        let test_cases = [1, 2, 4, 7, 8, 11];

        for number in test_cases {
            assert_eq!(is_multiple_of_three_or_five(number), false);
        }
    }

    #[test]
    fn should_sum() {
        assert_eq!(sum_of_all_multiple_of_three_or_five_below(10), 23)
    }
}
