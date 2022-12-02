use std::{fs, str::FromStr};

fn main() -> anyhow::Result<()> {
    let input = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&input));
    println!("Answer 2: {:?}", part_02(input));

    Ok(())
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

fn part_01(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|game| {
            let hands: Vec<Hand> = game
                .split_whitespace()
                .filter_map(|hand| hand.parse().ok())
                .collect();

            let result = get_game_result(&hands[1], &hands[0]);

            result.value() + hands[1].value()
        })
        .sum()
}

fn part_02(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|game| {
            let (hand, result) = game.split_once(' ').unwrap();

            let hand = hand.parse::<Hand>().unwrap();
            let result = result.parse::<GameResult>().unwrap();

            let yours = get_game_result_2(&result, &hand);

            result.value() + yours.value()
        })
        .sum()
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    fn value(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissor,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissor,
            _ => panic!("Unknown hand"),
        })
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    fn value(&self) -> usize {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

impl FromStr for GameResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("Unknown hand"),
        })
    }
}

fn get_game_result(yours: &Hand, opponent: &Hand) -> GameResult {
    match yours {
        Hand::Rock => match opponent {
            Hand::Rock => GameResult::Draw,
            Hand::Paper => GameResult::Lose,
            Hand::Scissor => GameResult::Win,
        },
        Hand::Paper => match opponent {
            Hand::Rock => GameResult::Win,
            Hand::Paper => GameResult::Draw,
            Hand::Scissor => GameResult::Lose,
        },
        Hand::Scissor => match opponent {
            Hand::Rock => GameResult::Lose,
            Hand::Paper => GameResult::Win,
            Hand::Scissor => GameResult::Draw,
        },
    }
}

fn get_game_result_2(result: &GameResult, opponent: &Hand) -> Hand {
    match result {
        GameResult::Win => match opponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissor,
            Hand::Scissor => Hand::Rock,
        },
        GameResult::Draw => match opponent {
            Hand::Rock => Hand::Rock,
            Hand::Paper => Hand::Paper,
            Hand::Scissor => Hand::Scissor,
        },
        GameResult::Lose => match opponent {
            Hand::Rock => Hand::Scissor,
            Hand::Paper => Hand::Rock,
            Hand::Scissor => Hand::Paper,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 15);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(input), 12);
    }
}
