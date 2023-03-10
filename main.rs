use std::vec;

fn main() {
    print!("number of steps: {:?} \n", number_of_steps(14));
    print!("running sum: {:?}\n", running_sum(vec![3, 1, 2, 10, 1]));
    print!("max ones: {:?}\n", max_consec_ones(vec![1, 0, 1, 1, 0, 1]));
    print!(
        "square and sort: {:?}\n",
        square_and_sort(vec![-7, -3, 2, 3, 11])
    );

    // Input: nums = [3,1,2,10,1]
    // Output: [3,4,6,16,17]
}

fn items_with_even_digits(input: Vec<i32>) -> i32 {
    input
        .iter()
        .filter(|item| item.to_string().len() % 2 == 0)
        .count() as i32
}

// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]

// Input:
// [-7,-3,2,3,11]
// Output:
// [0,9,9,49,121]
// Expected:
// [4,9,9,49,121]

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

// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
fn longest_palindrome(s: String) -> String {
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s[left..] == s[..right] {
            todo!()
        }
    }

    todo!()
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
