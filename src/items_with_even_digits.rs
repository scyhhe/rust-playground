// Input: nums = [3,1,2,10,1]
// Output: [3,4,6,16,17]
pub fn items_with_even_digits(input: Vec<i32>) -> i32 {
    input
        .iter()
        .filter(|item| item.to_string().len() % 2 == 0)
        .count() as i32
}
