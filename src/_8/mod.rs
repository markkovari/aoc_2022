use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug)]
struct Problem {
    trees: Vec<Vec<u8>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| u8::try_from(d).unwrap())
                            .ok_or_else(|| anyhow!("couldn't parse digit from '{}'", c))
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { trees })
    }
}

fn visible_trees_map(trees: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let rows_count = trees.len();
    let columns_count = trees[0].len();

    // Use simple Boolean mask, memory is cheap
    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; columns_count]; rows_count];

    for col in 0..columns_count {
        let mut latest_visible_tree_height = trees[0][col];
        visible_trees[0][col] = true;
        for row in 1..rows_count {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    for row in 0..rows_count {
        let mut latest_visible_tree_height = trees[row][columns_count - 1];
        visible_trees[row][columns_count - 1] = true;
        for col in (0..(columns_count - 1)).rev() {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    for col in 0..columns_count {
        let mut latest_visible_tree_height = trees[rows_count - 1][col];
        visible_trees[rows_count - 1][col] = true;
        for row in (0..(rows_count - 1)).rev() {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    for row in 0..rows_count {
        let mut latest_visible_tree_height = trees[row][0];
        visible_trees[row][0] = true;
        for col in 0..columns_count {
            if latest_visible_tree_height < trees[row][col] {
                visible_trees[row][col] = true;
                latest_visible_tree_height = trees[row][col];
            }
            if latest_visible_tree_height == 9 {
                break;
            }
        }
    }

    visible_trees
}

fn count_visible_trees(visible_trees: &Vec<Vec<bool>>) -> u64 {
    visible_trees
        .iter()
        .map(|v| {
            v.iter()
                .filter_map(|&b| if b { Some(1) } else { None })
                .sum::<u64>()
        })
        .sum()
}

fn compute_viewing_distances(tree_line: &[u8]) -> Vec<u8> {
    let mut scores: Vec<u8> = vec![0; tree_line.len()];

    for i in 0..tree_line.len() {
        for j in (i + 1)..tree_line.len() {
            if tree_line[j] < tree_line[i] && j != (tree_line.len() - 1) {
                continue;
            }
            scores[i] = (j - i) as u8;
            break;
        }
    }

    scores
}

fn compute_scenic_scores(trees: &Vec<Vec<u8>>) -> Vec<Vec<u64>> {
    let rows_count = trees.len();
    let columns_count = trees[0].len();

    let trees_transposed: Vec<Vec<u8>> = (0..columns_count)
        .map(|col| {
            let mut column: Vec<u8> = vec![0; rows_count];
            for row in 0..rows_count {
                column[row] = trees[row][col];
            }

            column
        })
        .collect();

    let mut scenic_scores: Vec<Vec<u64>> = vec![vec![1; columns_count]; rows_count];

    let eastward_viewing_distances = trees
        .iter()
        .map(|row| compute_viewing_distances(row))
        .collect::<Vec<_>>();

    let westward_viewing_distances = trees
        .iter()
        .cloned()
        .map(|row| {
            let mut scores = compute_viewing_distances(&row.into_iter().rev().collect::<Vec<_>>());
            scores.reverse();

            scores
        })
        .collect::<Vec<_>>();

    let southward_viewing_distances = trees_transposed
        .iter()
        .map(|column| compute_viewing_distances(&column))
        .collect::<Vec<_>>();

    let northward_viewing_distances = trees_transposed
        .iter()
        .cloned()
        .map(|column| {
            let mut scores =
                compute_viewing_distances(&column.into_iter().rev().collect::<Vec<_>>());
            scores.reverse();

            scores
        })
        .collect::<Vec<_>>();

    for i in 0..rows_count {
        for j in 0..columns_count {
            scenic_scores[i][j] *= eastward_viewing_distances[i][j] as u64;
            scenic_scores[i][j] *= westward_viewing_distances[i][j] as u64;
            scenic_scores[i][j] *= southward_viewing_distances[j][i] as u64;
            scenic_scores[i][j] *= northward_viewing_distances[j][i] as u64;
        }
    }

    scenic_scores
}

fn find_max(a: &Vec<Vec<u64>>) -> u64 {
    a.iter()
        .map(|r| r.iter().max().unwrap())
        .max()
        .unwrap()
        .clone()
}
pub fn get_8_first() -> u64 {
    let content = include_str!("../../inputs/8/input.data");
    let Problem { trees } = content.parse().unwrap();
    let visible_trees = visible_trees_map(&trees);

    count_visible_trees(&visible_trees)
}
pub fn get_8_second() -> u64 {
    let content = include_str!("../../inputs/8/input.data");
    let Problem { trees } = content.parse().unwrap();

    let scenic_scores = compute_scenic_scores(&trees);

    find_max(&scenic_scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_parsing_input() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            trees,
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ]
        );
    }

    #[test]
    fn test_visible_trees_map() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();
        let visible_trees = visible_trees_map(&trees);

        assert_eq!(
            visible_trees,
            vec![
                vec![true, true, true, true, true],
                vec![true, true, true, false, true],
                vec![true, true, false, true, true],
                vec![true, false, true, false, true],
                vec![true, true, true, true, true],
            ]
        );
    }

    #[test]
    fn test_count_visible_trees() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();
        let visible_trees = visible_trees_map(&trees);
        let count = count_visible_trees(&visible_trees);

        assert_eq!(count, 21);
    }

    #[test]
    fn test_compute_viewing_distances_1() {
        let tree_row = vec![2, 5, 5, 1, 2];
        let tree_col = vec![3, 5, 3, 5, 3];

        let eastward_scores = compute_viewing_distances(&tree_row);
        let mut westward_scores =
            compute_viewing_distances(&tree_row.into_iter().rev().collect::<Vec<_>>());
        westward_scores.reverse();

        let soutward_scores = compute_viewing_distances(&tree_col);
        let mut northward_scores =
            compute_viewing_distances(&tree_col.into_iter().rev().collect::<Vec<_>>());
        northward_scores.reverse();

        assert_eq!(eastward_scores, vec![1, 1, 2, 1, 0]);
        assert_eq!(westward_scores, vec![0, 1, 1, 1, 2]);
        assert_eq!(soutward_scores, vec![1, 2, 1, 1, 0]);
        assert_eq!(northward_scores, vec![0, 1, 1, 2, 1]);
    }

    #[test]
    fn test_compute_viewing_distances_2() {
        let tree_row = vec![3, 3, 5, 4, 9];
        let tree_col = vec![3, 5, 3, 5, 3];

        let eastward_scores = compute_viewing_distances(&tree_row);
        let mut westward_scores =
            compute_viewing_distances(&tree_row.into_iter().rev().collect::<Vec<_>>());
        westward_scores.reverse();

        let soutward_scores = compute_viewing_distances(&tree_col);
        let mut northward_scores =
            compute_viewing_distances(&tree_col.into_iter().rev().collect::<Vec<_>>());
        northward_scores.reverse();

        assert_eq!(eastward_scores, vec![1, 1, 2, 1, 0]);
        assert_eq!(westward_scores, vec![0, 1, 2, 1, 4]);
        assert_eq!(soutward_scores, vec![1, 2, 1, 1, 0]);
        assert_eq!(northward_scores, vec![0, 1, 1, 2, 1]);
    }

    #[test]
    fn test_compute_scenic_score() {
        let Problem { trees } = TEST_INPUT.parse().unwrap();
        let scores = compute_scenic_scores(&trees);

        assert_eq!(scores[1][2], 4);
        assert_eq!(scores[3][2], 8);
        assert_eq!(scores[4][3], 0);
        assert_eq!(find_max(&scores), 8);
    }
}
