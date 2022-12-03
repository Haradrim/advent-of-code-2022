use std::{collections::HashSet, fs};

fn main() -> anyhow::Result<()> {
    let input = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&input));
    println!("Answer 2: {:?}", part_02(&input));

    Ok(())
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

fn part_01(rucksacks: &Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);

            let set: HashSet<char> = compartment_1.chars().collect();

            let char_found = compartment_2.chars().find(|char| set.contains(char));

            match char_found {
                Some(char) => calc_item_priority(&char),
                None => 0,
            }
        })
        .sum()
}

fn part_02(rucksacks: &Vec<String>) -> u32 {
    rucksacks
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
                .reduce(|acc, rucksack_content| {
                    acc.intersection(&rucksack_content).copied().collect()
                })
                .unwrap()
                .iter()
                .map(|char| calc_item_priority(char))
                .sum::<u32>()
        })
        .sum()
}

fn calc_item_priority(char: &char) -> u32 {
    let ascii = char.clone() as u32;

    if char.is_lowercase() {
        return ascii - 96;
    };

    ascii - 38
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_score() {
        assert_eq!(calc_item_priority(&'p'), 16);
        assert_eq!(calc_item_priority(&'L'), 38);
        assert_eq!(calc_item_priority(&'P'), 42);
        assert_eq!(calc_item_priority(&'v'), 22);
        assert_eq!(calc_item_priority(&'t'), 20);
        assert_eq!(calc_item_priority(&'s'), 19);
    }

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 157);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), 70);
    }
}
