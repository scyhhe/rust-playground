pub fn is_palindrome(x: i32) -> bool {
    let num_as_string = x.to_string();
    let chars = num_as_string.as_bytes();
    let mut left = 0;

    let mut right = x.to_string().len() - 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
