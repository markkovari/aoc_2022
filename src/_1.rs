pub fn get_1_first() -> i32 {
    let content = include_str!("../inputs/1/input.data");
    let mut max_sum: i32 = 0;

    for block in content.split("\n\n") {
        let current_block_sum = block
            .split("\n")
            .into_iter()
            .filter_map(|e| match e.parse::<i32>() {
                Ok(e) => Some(e),
                Err(_) => None,
            })
            .fold(0, |a, b| a + b);
        if current_block_sum > max_sum {
            max_sum = current_block_sum;
        }
    }

    max_sum
}

pub fn get_1_second() -> i32 {
    let content = include_str!("../inputs/1/input.data");
    let first_x_amount = 3;
    let mut calorie_sums = content
        .split("\n\n")
        .map(|block| {
            block
                .split("\n")
                .into_iter()
                .filter_map(|e| match e.parse::<i32>() {
                    Ok(e) => Some(e),
                    Err(_) => None,
                })
                .fold(0, |a, b| a + b)
        })
        .collect::<Vec<i32>>();
    calorie_sums.sort();
    calorie_sums.reverse();
    calorie_sums[0..first_x_amount].iter().fold(0, |a, b| a + b)
}
