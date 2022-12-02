// (0 if you lost, 3 if the round was a draw, and 6 if you won)

fn get_choice_value(choice: &str) -> i32 {
    match choice {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn get_status(choice: &str) -> i32 {
    match choice {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}
fn get_compared_value(opponent: &str, choice: &str) -> i32 {
    match (choice, opponent) {
        ("A", ours) => match ours {
            "Y" => 6,
            "X" => 3,
            _ => 0,
        },
        ("B", ours) => match ours {
            "Z" => 6,
            "Y" => 3,
            _ => 0,
        },
        ("C", ours) => match ours {
            "X" => 6,
            "Z" => 3,
            _ => 0,
        },
        (_, _) => 0,
    }
}

// opponent A for Rock, B for Paper, and C for Scissors
// ours X for Rock, Y for Paper, and Z for Scissors
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
fn get_choice_based_on_expected_outcome<'a>(opponent: &'a str, choice: &'a str) -> &'a str {
    match (choice, opponent) {
        ("A", ours) => match ours {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => "",
        },
        ("B", ours) => match ours {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => "",
        },
        ("C", ours) => match ours {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => "",
        },
        (_, _) => "",
    }
}

pub fn get_2_first() -> i32 {
    let line_content = include_str!("../2_input.data").lines();

    line_content
        .into_iter()
        .map(|line| {
            let chars = line.split(" ").collect::<Vec<&str>>();
            return get_choice_value(chars[1]) + get_compared_value(chars[1], chars[0]);
        })
        .fold(0, |a, b| a + b)
}

pub fn get_2_second() -> i32 {
    let line_content = include_str!("../2_input.data").lines();

    line_content
        .into_iter()
        .map(|line| {
            let chars = line.split(" ").collect::<Vec<&str>>();
            let picked_for_outcome = get_choice_based_on_expected_outcome(chars[1], chars[0]);
            let choice_value = get_choice_value(picked_for_outcome);
            let outcome_of_round = get_status(chars[1]);
            return choice_value + outcome_of_round;
        })
        .fold(0, |a, b| a + b)
}
