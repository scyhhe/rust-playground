// Input: arr = [1,0,2,3,0,4,5,0]
// Output: [1,0,0,2,3,0,0,4]
// Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
pub fn duplicate_zeros(arr: &mut Vec<i32>) -> Vec<i32> {
    let max_length = arr.len() - 1;
    let mut cursor = 0;

    while cursor <= max_length {
        if arr[cursor] == 0 {
            arr.insert(cursor, 0);
            cursor += 1;
        }
        cursor += 1;
    }
    arr.as_slice()[..=max_length].to_vec()
}
