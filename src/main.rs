#![allow(dead_code)]

use crate::square_and_sort::square_and_sort;

mod duplicate_zeros;
mod is_palindrome;
mod items_with_even_digits;
mod max_consecutive_ones;
mod number_of_steps;
mod running_sum;
mod square_and_sort;

fn main() {
    let square_and_sort_result = square_and_sort(vec![-4, -1, 0, 3, 10]);
    let expected = vec![0, 1, 9, 16, 100];
    assert_eq!(square_and_sort_result, expected)
}
