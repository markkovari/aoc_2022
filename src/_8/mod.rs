type ElementType = u64;

fn read_into_2d(content: String) -> Vec<Vec<ElementType>> {
    content
        .lines()
        .into_iter()
        .map(|line| {
            line.split("")
                .into_iter()
                .filter_map(|e| match e.parse::<ElementType>() {
                    Ok(e) => Some(e),
                    Err(_) => None,
                })
                .collect()
        })
        .collect()
}

fn get_column_at(at: usize, elements: Vec<Vec<ElementType>>) -> Vec<ElementType> {
    elements
        .iter()
        .map(|row| *row.get(at).unwrap())
        .collect::<Vec<ElementType>>()
}

fn is_highest(at_x: usize, at_y: usize, elements: Vec<Vec<ElementType>>) -> bool {
    let row = elements.get(at_x).unwrap();
    let column = get_column_at(at_y, elements.clone());
    is_visible(at_x, column.clone()) || is_visible(at_y, row.to_vec())
}

fn get_frame_amount(length: u64) -> u64 {
    2 * length + (length - 2) * 2
}

fn calculate_highest_amount(hights: Vec<Vec<ElementType>>) -> u64 {
    let mut count_of_highest = 0;
    let length = hights.len();
    for i in 0..length {
        let current_row_length = hights[i].len();
        for j in 0..current_row_length {
            if is_highest(i, j, hights.clone()) {
                count_of_highest += 1;
            } else {
                println!("{} {} {}", i, j, hights[i][j]);
            }
        }
    }

    count_of_highest
}

fn is_visible(at: usize, of: Vec<u64>) -> bool {
    if at == 0 || of.len() - 1 == at {
        return true;
    }
    if let Some(value_at) = of.get(at) {
        return of[0..at].iter().all(|e| e < value_at)
            || of[at + 1..of.len()].iter().all(|e| e < value_at);
    }
    return false;
}

pub fn get_8_first() -> u64 {
    let content = include_str!("../../inputs/8/input.data").to_owned();

    let numbers = read_into_2d(content);
    calculate_highest_amount(numbers)
}
pub fn get_8_second() -> u64 {
    3
}

#[cfg(test)]
mod test_8 {
    use super::*;

    #[test]
    fn read_into_2d_test() {
        let elements_of_example =
            read_into_2d(include_str!("../../inputs/8/input.example").to_owned());
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(elements_of_example, expected);
    }

    #[test]
    fn outers_are_visible() {
        let numbers = vec![0, 1, 0];
        assert!(is_visible(2, numbers.clone()));
        assert!(is_visible(0, numbers));
    }

    #[test]
    fn only_inner_are_visible() {
        let numbers = vec![0, 1, 0];
        assert!(is_visible(1, numbers));
    }

    #[test]
    fn from_two_inner_corrects_are_visible() {
        let numbers = vec![3, 1, 3, 0];
        assert!(is_visible(0, numbers.clone()));
        assert!(!is_visible(1, numbers.clone()));
        assert!(is_visible(2, numbers.clone()));
        assert!(is_visible(3, numbers.clone()));
    }

    #[test]
    fn from_two_inner_none_is_visible_when_outers_are_9s() {
        let numbers = vec![9, 1, 3, 9];
        assert!(is_visible(0, numbers.clone()));
        assert!(!is_visible(1, numbers.clone()));
        assert!(!is_visible(2, numbers.clone()));
        assert!(is_visible(3, numbers.clone()));
    }

    #[test]
    fn two_by_two_amount_is_four() {
        let numbers = vec![vec![1, 1], vec![1, 1]];
        let higher_ones = calculate_highest_amount(numbers);
        assert_eq!(higher_ones, 4);
    }

    #[test]
    fn test_anomaly() {
        let numbers = vec![vec![1, 0, 0, 1], vec![1, 0, 0, 1]];
        assert_eq!(calculate_highest_amount(numbers), 4);
    }

    #[test]
    fn example_amount_is_correct() {
        let numbers = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let higher_ones = calculate_highest_amount(numbers);
        assert_eq!(higher_ones, 21);
    }

    #[test]
    fn get_column_gets_back_the_column() {
        let numbers = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let first_column = get_column_at(0, numbers.clone());
        let last_column = get_column_at(4, numbers);
        assert_eq!(first_column, vec![3, 2, 6, 3, 3]);
        assert_eq!(last_column, vec![3, 2, 2, 9, 0]);
    }

    #[test]
    fn no_inner_element_is_higher() {
        let expected = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 8);
    }

    #[test]
    fn inner_element_is_higher() {
        let expected = vec![vec![1, 1, 1], vec![1, 2, 1], vec![1, 1, 1]];
        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 9);
    }

    #[test]
    fn inner_element_is_higher_four_times_four_no_inner_higher_element() {
        let expected = vec![
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 12);
    }

    #[test]
    fn inner_element_is_higher_four_times_four_one_inner_higher_element() {
        let expected = vec![
            vec![1, 1, 1, 1],
            vec![1, 4, 0, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 13);
    }

    #[test]
    fn inner_element_is_higher_four_times_four_two_inner_higher_element() {
        let expected = vec![
            vec![1, 1, 1, 1],
            vec![1, 2, 2, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];

        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 14);
    }

    #[test]
    fn asymmetric_matrix() {
        let expected = vec![
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
            vec![1, 2, 3, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 18);
    }

    #[test]
    fn inner_element_is_higher_four_times_four_four_inner_higher_element() {
        let expected = vec![
            vec![1, 1, 1, 1],
            vec![1, 2, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 1, 1, 1],
        ];

        let high_amount = calculate_highest_amount(expected);
        assert_eq!(high_amount, 16);
    }
}
