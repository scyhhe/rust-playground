pub fn max_consecutive_ones(input: Vec<i32>) -> i32 {
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
