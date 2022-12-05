use std::{fs, str::FromStr};

fn main() -> anyhow::Result<()> {
    let input = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&input));
    println!("Answer 2: {:?}", part_02(&input));

    Ok(())
}

fn get_stacks(input: &[String]) -> Vec<Vec<char>> {
    let (ids, stacks) = input.split_last().unwrap();

    ids.chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| {
            stacks
                .iter()
                .map(|s| s.chars().collect::<Vec<char>>()[i])
                .filter(|c| !c.is_whitespace())
                .collect()
        })
        .collect()
}

fn part_01(input: &Vec<String>) -> String {
    let index = input.iter().position(|x| x == "").unwrap();
    let (stacks, instructions) = input.split_at(index);

    let mut stacks = get_stacks(stacks);

    instructions
        .iter()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<Instruction>().ok())
        .for_each(|instruction| {
            let removed: Vec<char> = stacks[instruction.from - 1]
                .drain(..instruction.quantity)
                .collect();

            removed
                .iter()
                .for_each(|c| stacks[instruction.to - 1].insert(0, *c));
        });

    stacks.iter().map(|c| c[0]).collect()
}

fn part_02(input: &Vec<String>) -> String {
    let index = input.iter().position(|x| x == "").unwrap();
    let (stacks, instructions) = input.split_at(index);

    let mut stacks = get_stacks(stacks);

    instructions
        .iter()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<Instruction>().ok())
        .for_each(|instruction| {
            let removed: Vec<char> = stacks[instruction.from - 1]
                .drain(..instruction.quantity)
                .collect();

            removed
                .iter()
                .rev()
                .for_each(|c| stacks[instruction.to - 1].insert(0, *c));
        });

    stacks.iter().map(|c| c[0]).collect()
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<usize> = s
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Instruction {
            quantity: input[0],
            from: input[1],
            to: input[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), "CMZ");
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), "MCD");
    }
}
