pub fn number_of_steps(num: i32) -> i32 {
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
