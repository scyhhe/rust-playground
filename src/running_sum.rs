pub fn running_sum(input: Vec<i32>) -> Vec<i32> {
    input
        .into_iter()
        .scan(0, |agg, curr| {
            *agg += curr;
            Some(*agg)
        })
        .collect()
}
