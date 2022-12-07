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

fn get_start_of_packet_marker(input: &String, offset: usize) -> usize {
    let mut sequence_length: usize = 0;
    let chars = input.chars().collect::<Vec<char>>();

    for (index, window) in chars.windows(offset).enumerate() {
        if window.iter().collect::<HashSet<&char>>().len() == offset {
            sequence_length = index + offset;
            break;
        }
    }

    sequence_length
}

fn part_01(input: &Vec<String>) -> usize {
    get_start_of_packet_marker(&input[0], 4)
}

fn part_02(input: &Vec<String>) -> usize {
    get_start_of_packet_marker(&input[0], 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 7);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), 7);
    }
}
