use anyhow::Result;
use std::fs::{self};

fn main() -> Result<()> {
    let input = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&input));

    println!("Answer 2: {:?}", part_02(&input));

    Ok(())
}

fn part_01(input: &Vec<u32>) -> Result<Option<&u32>> {
    Ok(input.iter().max())
}

fn part_02(input: &Vec<u32>) -> Result<u32> {
    let mut copy = input.clone();

    copy.sort_by(|a, b| b.cmp(a));

    Ok(copy.iter().take(3).sum::<u32>())
}

fn read_file(filename: &str) -> std::io::Result<Vec<u32>> {
    let input = fs::read_to_string(filename)?;

    let result = input
        .split("\r\n\r\n")
        .map(|entry| {
            entry
                .lines()
                .flat_map(|calories| calories.parse::<u32>())
                .sum::<u32>()
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        let result = part_01(&input).unwrap().unwrap().clone();

        assert_eq!(result, 24000);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        let result = part_02(&input).unwrap();

        assert_eq!(result, 45000);
    }
}
