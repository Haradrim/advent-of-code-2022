use std::fs;

fn main() -> anyhow::Result<()> {
    let input = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&input));
    println!("Answer 2: {:?}", part_02(&input));

    Ok(())
}

fn count_trees(input: &Vec<String>) -> Vec<Vec<isize>> {
    let trees: Vec<Vec<isize>> = input
        .iter()
        .map(|row| {
            row.chars()
                .filter_map(|tree| tree.to_string().parse().ok())
                .collect()
        })
        .collect();

    let height = trees.len();
    let width = trees[0].len();
    let mut seen_count: Vec<Vec<isize>> = vec![vec![0; width]; height];
    for y in 0..height {
        let mut max_height_right = -1;
        let mut max_height_left = -1;

        for x in 0..width {
            let right = trees[y][x];
            let left = trees[y][width - x - 1];

            if right > max_height_right {
                seen_count[y][x] += 1;
                max_height_right = right;
            }

            if left > max_height_left {
                seen_count[y][width - x - 1] += 1;
                max_height_left = left;
            }
        }
    }
    for x in 0..width {
        let mut max_height_top = -1;
        let mut max_height_bottom = -1;

        for y in 0..height {
            let top = trees[y][x];
            let bottom = trees[height - y - 1][x];

            if top > max_height_top {
                seen_count[y][x] += 1;
                max_height_top = top;
            }

            if bottom > max_height_bottom {
                seen_count[height - y - 1][x] += 1;
                max_height_bottom = bottom;
            }
        }
    }
    seen_count
}

fn calculate_view_score(trees: &Vec<Vec<isize>>, x: usize, y: usize) -> usize {
    let height = trees.len();
    let width = trees[0].len();

    let tree_height = trees[y][x];

    let mut dist_left = 0;
    let mut dist_right = 0;
    let mut dist_up = 0;
    let mut dist_down = 0;

    for x in (0..x).rev() {
        if trees[y][x] < tree_height {
            dist_left += 1;
        } else if trees[y][x] >= tree_height {
            dist_left += 1;
            break;
        } else {
            break;
        }
    }

    for x in x + 1..width {
        if trees[y][x] < tree_height {
            dist_right += 1;
        } else if trees[y][x] >= tree_height {
            dist_right += 1;
            break;
        } else {
            break;
        }
    }

    for y in (0..y).rev() {
        if trees[y][x] < tree_height {
            dist_up += 1;
        } else if trees[y][x] >= tree_height {
            dist_up += 1;
            break;
        } else {
            break;
        }
    }

    for y in y + 1..height {
        if trees[y][x] < tree_height {
            dist_down += 1;
        } else if trees[y][x] >= tree_height {
            dist_down += 1;
            break;
        } else {
            break;
        }
    }

    1 * dist_left * dist_right * dist_up * dist_down
}

fn part_01(input: &Vec<String>) -> usize {
    let tree_count = count_trees(&input);

    tree_count
        .iter()
        .flat_map(|row| row.iter().filter(|count| **count != 0))
        .count()
}

fn part_02(input: &Vec<String>) -> usize {
    let trees: Vec<Vec<isize>> = input
        .iter()
        .map(|row| {
            row.chars()
                .filter_map(|tree| tree.to_string().parse().ok())
                .collect()
        })
        .collect();

    trees
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| calculate_view_score(&trees, x, y))
                .collect::<Vec<usize>>()
        })
        .max()
        .unwrap()
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 21);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), 8);
    }
}
