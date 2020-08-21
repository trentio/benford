// Does a text breakdown of the data given via println
use std::collections::HashMap;
use itertools::Itertools;

pub fn println_breakdown(digits: &Vec<usize>) {
    let breakdown: HashMap<&usize, Vec<usize>> =
        digits
            .into_iter()
            .map(|v| (v, 1))
            .into_group_map()
            ;

    let number_of_values = digits.len();

    println!("Breakdown:");
    for (digit, list) in breakdown.iter().sorted() {
        let total = list.len();
        let percent_of_total_values: f64 = total as f64 / number_of_values as f64;

        println!("Digit {}: {} total, {:.2} percent.", digit, total, percent_of_total_values)
    }
    println!("Total items: {}", number_of_values);
}