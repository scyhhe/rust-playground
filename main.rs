#![allow(dead_code)]

use std::vec;

fn main() {
    println!("number of steps: {:?} ", number_of_steps(14));
    println!("running sum: {:?}", running_sum(vec![3, 1, 2, 10, 1]));
    println!("max ones: {:?}", max_consec_ones(vec![1, 0, 1, 1, 0, 1]));
    println!(
        "square and sort: {:?}",
        square_and_sort(vec![-7, -3, 2, 3, 11])
    );
    println!(
        "dup zeroes: {:?}",
        duplicate_zeros(&mut vec![1, 0, 2, 3, 0, 4, 5, 0])
    );
}

// Input: nums = [3,1,2,10,1]
// Output: [3,4,6,16,17]
fn items_with_even_digits(input: Vec<i32>) -> i32 {
    input
        .iter()
        .filter(|item| item.to_string().len() % 2 == 0)
        .count() as i32
}

// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
fn square_and_sort(input: Vec<i32>) -> Vec<i32> {
    let length = input.len();
    let mut left = 0;
    let mut right = length - 1;

    let mut sort_pointer = length - 1;

    let mut result = vec![0; length];

    while right >= left {
        let left_num = input[left];
        let right_num = input[right];

        if left_num.abs() > right_num {
            result[sort_pointer] = left_num * left_num;
            left += 1;
        } else {
            result[sort_pointer] = right_num * right_num;
            right -= 1;
        }

        if sort_pointer == 0 {
            break;
        }
        sort_pointer -= 1;
    }

    result
}

fn max_consec_ones(input: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut temp = 0;

    for item in input {
        if item == 0 {
            res = res.max(temp);
            temp = 0;
        } else {
            temp += 1;
        }
    }

    res.max(temp)
}

// Input: arr = [1,0,2,3,0,4,5,0]
// Output: [1,0,0,2,3,0,0,4]
// Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
fn duplicate_zeros(arr: &mut Vec<i32>) -> Vec<i32> {
    let max_length = arr.len() - 1;
    let mut cursor = 0;

    while cursor <= max_length {
        if arr[cursor] == 0 {
            arr.insert(cursor, 0);
            cursor += 1;
        }
        cursor += 1;
    }
    arr[..=max_length].to_vec()
}

// TODO do it properly with pointers
// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// Output: [1,2,2,3,5,6]
// Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
// The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
    let mut a = nums1[..m as usize].to_vec();
    let mut b = nums2[..n as usize].to_vec();

    a.append(&mut b);

    a.sort();

    a
}

fn running_sum(input: Vec<i32>) -> Vec<i32> {
    input
        .into_iter()
        .scan(0, |agg, curr| {
            *agg += curr;
            Some(*agg)
        })
        .collect()
}

fn number_of_steps(num: i32) -> i32 {
    fn count(n: i32, steps: i16) -> i16 {
        if n == 0 {
            steps
        } else {
            match n % 2 {
                0 => count(n / 2, steps + 1),
                1 => count(n - 1, steps + 1),
                _ => steps,
            }
        }
    }
    count(num, 0) as i32
}
