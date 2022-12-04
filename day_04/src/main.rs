use std::{fs, ops::RangeInclusive};

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

fn get_range(elf: &str) -> RangeInclusive<i32> {
    let (lower, upper) = elf.split_once('-').expect("Invalid input");

    let lower: i32 = lower.parse().unwrap();
    let upper: i32 = upper.parse().unwrap();

    lower..=upper
}

fn part_01(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|pair| {
            let (elf1, elf2) = pair.split_once(',').expect("Invalid input");

            let elf_range1 = get_range(elf1);
            let elf_range2 = get_range(elf2);

            elf_range1.clone().all(|x| elf_range2.contains(&x))
                || elf_range2.clone().all(|x| elf_range1.contains(&x))
        })
        .count()
}

fn part_02(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|pair| {
            let (elf1, elf2) = pair.split_once(',').expect("Invalid input");

            let elf_range1 = get_range(elf1);
            let elf_range2 = get_range(elf2);

            elf_range1.clone().any(|x| elf_range2.contains(&x))
                || elf_range2.clone().any(|x| elf_range1.contains(&x))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 2);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), 4);
    }
}
