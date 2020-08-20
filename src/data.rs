extern crate rand;

use rand::Rng;
use std::convert::TryInto;
use std::str::FromStr;

pub fn gen_random_num_vector(len: isize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen::<isize>()).collect()
}

pub fn tallest_buildings() -> Vec<isize> {
    // heights pulled from wiki page on tall buildings
    let heights = vec![839, 640, 634, 629,604, 601, 529, 472, 458,452, 442, 420, 400, 385, 370, 367, 354, 342, 314, 305, 291, 274, 265, 262, 248, 247, 240, 214, 202, 192, 187, 178, 175, 174, 172, 171, 167, 167, 162, 160, 152, 150, 139, 139, 141, 139, 138, 122, 118, 118, 115, 114, 100, 96, 85, 82, 76];
    println!("There are {} heights", heights.len());
    heights
}

pub fn transaction_amounts() -> Vec<isize> {
    vec![1.1].iter().map(|&x| ignore_the_decimal(x)).collect()
}

fn ignore_the_decimal(num: f64) -> isize {
    isize::from_str(&num.to_string().replace(".", "")).unwrap_or(0)
}

pub fn first_digit(num: isize) -> usize {
    if num / 10 == 0 {
        return num.abs().try_into().unwrap();
    } else {
        return first_digit(num / 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_decimal_works() {
        assert_eq!(ignore_the_decimal(12.123), 12123);
        assert_eq!(ignore_the_decimal(0.01), 1);
    }

    #[test]
    fn finds_first_digits_works() {
        assert_eq!(first_digit(1234), 1);
        assert_eq!(first_digit(345), 3);
        assert_eq!(first_digit(8), 8);
    }

    #[test]
    fn find_first_handles_negative() {
        assert_eq!(first_digit(-1234), 1);
        assert_eq!(first_digit(-9), 9);
    }

    #[test]
    fn find_first_handles_zero() {
        assert_eq!(first_digit(0), 0);
    }
}
