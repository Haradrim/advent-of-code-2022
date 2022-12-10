use std::{collections::BTreeMap, fs, path::PathBuf, str::FromStr};

fn main() -> anyhow::Result<()> {
    let input = read_file("input.txt")?;

    println!("Answer 1: {:?}", part_01(&input));
    println!("Answer 2: {:?}", part_02(&input));

    Ok(())
}

fn part_01(input: &Vec<String>) -> usize {
    let commands: Vec<Command> = input.iter().filter_map(|c| c.parse().ok()).collect();

    let file_tree = calc_directory_sizes(commands);

    file_tree
        .iter()
        .filter_map(|(_, entry)| match entry {
            DirectoryContent::Directory(dir) => {
                if dir.size <= 100_000 {
                    return Some(dir.size);
                }

                None
            }
            _ => None,
        })
        .sum()
}

fn part_02(input: &Vec<String>) -> usize {
    let commands: Vec<Command> = input.iter().filter_map(|c| c.parse().ok()).collect();

    let file_tree = calc_directory_sizes(commands);

    let available_space = 70000000;
    let required_space = 30000000;
    let used_space = match &file_tree["/"] {
        DirectoryContent::Directory(dir) => dir.size,
        _ => panic!("Should be directory"),
    };
    let unused_space = available_space - used_space;
    let delete_space = required_space - unused_space;

    file_tree
        .iter()
        .filter_map(|(_, entry)| match entry {
            DirectoryContent::Directory(dir) => {
                if dir.size >= delete_space {
                    return Some(dir.size);
                }

                None
            }
            _ => None,
        })
        .min()
        .unwrap()
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().filter_map(|line| line.parse().ok()).collect())
}

fn calc_directory_sizes(commands: Vec<Command>) -> BTreeMap<String, DirectoryContent> {
    let mut path = PathBuf::new();
    let mut file_tree = BTreeMap::<String, DirectoryContent>::new();

    file_tree.insert(
        "/".to_string(),
        DirectoryContent::Directory(Directory {
            name: "/".to_string(),
            size: 0,
        }),
    );

    commands
        .iter()
        .for_each(|command| command.execute(&mut file_tree, &mut path));

    let mut files = file_tree
        .iter()
        .map(|(path, entry)| (path.clone(), entry.clone()))
        .collect::<Vec<_>>();

    files.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

    files
        .iter()
        .filter_map(|(path, entry)| match entry {
            DirectoryContent::Directory(_) => Some(path.clone()),
            _ => None,
        })
        .for_each(|dir| {
            let size = files
                .iter()
                .filter_map(|(path, entry)| {
                    (path.starts_with(&dir)).then_some(match entry {
                        DirectoryContent::File(file) => file.size,
                        DirectoryContent::Directory(dir) => dir.size,
                    })
                })
                .sum();

            match file_tree.get_mut(&dir).unwrap() {
                DirectoryContent::File(_) => panic!("Should not be a file"),
                DirectoryContent::Directory(dir) => dir.size = size,
            }
        });

    file_tree
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum ChangeDirectory {
    Destination(String),
    Up,
}

impl FromStr for ChangeDirectory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ".." => ChangeDirectory::Up,
            _ => ChangeDirectory::Destination(s.to_string()),
        })
    }
}

#[derive(Debug, Clone)]
enum DirectoryContent {
    File(File),
    Directory(Directory),
}

impl FromStr for DirectoryContent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split_whitespace().collect();

        let input_type = input[0].parse::<usize>();

        Ok(match input_type {
            Ok(size) => DirectoryContent::File(File {
                size,
                name: input[1].to_string(),
            }),
            Err(_) => match input[0] {
                "dir" => DirectoryContent::Directory(Directory {
                    name: input[1].to_string(),
                    size: 0,
                }),
                _ => panic!("Unknown content {:?}", input),
            },
        })
    }
}

#[derive(Debug)]
enum Command {
    Output(DirectoryContent),
    Cd(ChangeDirectory),
    Ls,
}

impl Command {
    fn execute(
        &self,
        file_tree: &mut BTreeMap<String, DirectoryContent>,
        path: &mut PathBuf,
    ) -> () {
        match self {
            Command::Output(content) => match content {
                DirectoryContent::File(file) => {
                    let mut path = path.clone();
                    path.push(file.name.clone());
                    file_tree.insert(path.display().to_string(), content.clone());
                }
                DirectoryContent::Directory(dir) => {
                    let mut path = path.clone();
                    path.push(dir.name.clone());
                    file_tree.insert(path.display().to_string(), content.clone());
                }
            },
            Command::Cd(c) => match c {
                ChangeDirectory::Destination(d) => {
                    path.push(d);
                }
                ChangeDirectory::Up => {
                    path.pop();
                }
            },
            Command::Ls => return,
        }
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split_whitespace().collect();

        if input[0] != "$" {
            return Ok(Command::Output(s.parse().unwrap()));
        };

        Ok(match input[1] {
            "cd" => Command::Cd(input[2].parse().unwrap()),
            "ls" => Command::Ls,
            _ => panic!("Unknown command {:?}", input[1]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 95437);
    }

    #[test]
    fn example_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), 24933642);
    }
}
