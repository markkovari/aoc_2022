pub fn get_1_first() -> i32 {
    let content = include_str!("../1_input.data");
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
    let content = include_str!("../1_input.data");

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
    let result = match calorie_sums[..] {
        [] => 0,
        [first] => first,
        [first, second] => first + second,
        [first, second, third] => first + second + third,
        [first, second, third, ..] => first + second + third,
    };
    result
}
