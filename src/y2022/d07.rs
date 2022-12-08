//! Day 7: No Space Left On Device

use std::collections::{HashMap, HashSet};

/// total size of directories with each a size of at most 100000
pub fn a(input: &Vec<&str>) -> String {
    const LIMIT: usize = 100000;
    let directories = parse_input(input);
    directories
        .iter()
        .map(|(_, dir)| dir.size)
        .filter(|size| *size <= LIMIT)
        .sum::<usize>()
        .to_string()
}

/// size of smallest directory to subtract from total to free up 30M
pub fn b(input: &Vec<&str>) -> String {
    const SPACE_AVAILABLE: usize = 70000000;
    const SPACE_REQUIRED: usize = 30000000;
    let directories = parse_input(input);
    let space_used = directories["/"].size;
    let space_unused = SPACE_AVAILABLE - space_used;
    let min_size = SPACE_REQUIRED - space_unused;
    directories
        .iter()
        .map(|(_, dir)| dir.size)
        .filter(|size| *size >= min_size)
        .min()
        .unwrap()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Directories {
    let mut directories = Directories::new();
    let mut path = Vec::<&str>::new();
    for line in input {
        let terms = line.split_whitespace().collect::<Vec<_>>();
        if terms[0] == "$" {
            match terms[1] {
                "cd" => {
                    if terms[2] == ".." {
                        update_size(&format!("{}", path.join("/")), &mut directories);
                        path.pop();
                    } else {
                        assert!(terms[2] != "/" || path.is_empty());
                        path.push(terms[2]);
                        let dir_path = format!("{}", path.join("/"));
                        directories
                            .entry(dir_path.to_owned())
                            .or_insert(Directory::default());
                    }
                }
                "ls" => {}
                _ => unreachable!("unknown command"),
            }
        } else {
            // directory list item
            let dir_path = path.join("/");
            let dir = directories.get_mut(&dir_path).unwrap();
            if terms[0] == "dir" {
                dir.subdirs
                    .insert(format!("{dir_path}/{}", terms[1]).to_owned());
            } else {
                dir.files
                    .push((terms[0].parse().unwrap(), terms[1].to_owned()));
            }
        }
    }
    while !path.is_empty() {
        update_size(&path.join("/"), &mut directories);
        path.pop();
    }
    directories
}

fn update_size(dir_name: &str, directories: &mut Directories) {
    let dir = directories.get(dir_name).unwrap();
    let file_size = dir.files.iter().fold(0, |acc, (size, _)| acc + size);
    let dir_size = dir.subdirs.iter().fold(0, |acc, name| {
        acc + directories.get(name).map_or(0, |dir| dir.size)
    });
    let size = file_size + dir_size;
    directories.get_mut(dir_name).unwrap().size = size;
}

#[derive(Debug, Default)]
struct Directory {
    subdirs: HashSet<String>,
    files: Vec<(usize, String)>,
    size: usize,
}

type Directories = HashMap<String, Directory>;

#[test]
pub fn test() {
    let input = vec![
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    assert_eq!(a(&input), "95437");
    assert_eq!(b(&input), "24933642");
}
