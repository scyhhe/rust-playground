// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
pub fn square_and_sort(input: Vec<i32>) -> Vec<i32> {
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
