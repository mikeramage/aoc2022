use crate::utils;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// The "linked file system nodes" design I initially came up with is fundamentally very tricky to get working in Rust.
// The problem is with lifetimes and mutability. Getting a FSNode to have a reference to its parent and modify that parent
// (e.g. to update its children as we discover them) is awkward to say the least!
//
// #[derive(Debug)]
// struct FSNode {
//     name: String,
//     children: Vec<FSNode>,
//     size: usize,
// }

// impl FSNode {
//     pub fn new(name: String, children: Vec<FSNode>, size: usize) -> FSNode {
//         let fsnode = FSNode {
//             name: name,
//             children: children,
//             size: size,
//         };
//         fsnode
//     }
//     pub fn add_child(&mut self, name: &str) {
//         self.children.push(FSNode{name: name.to_string(), children: vec!(), size: 0});
//     }
// }

// So instead we'll have 2 hashmaps:
// nodes - String (directory name) to Value (hashset of string child directory names and filenames).
// sizes - String (filename) to Value (size)
// We'll build these up in the pass through the input, then for each key in directories, we'll find all the files and sum them up.
// As we size directories, we can add to the sizes table so on future passes we'll be able to short-cut the traversal.

///Day 7 solution
pub fn day7() -> (usize, usize) {
    let lines: Vec<String> = utils::parse_input("input/day7.txt");
    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    construct_filesystem(&lines, &mut nodes, &mut sizes);
    // Calculating the size of the root node fills in all the other node sizes
    calculate_size(&nodes, &mut sizes, &mut dir_sizes, "/");

    let mut part1: usize = 0;
    let mut part2: usize = 0;

    let space_remaining = 70000000 - dir_sizes.get("/").unwrap();
    let space_required: usize = 30000000;
    let shortfall = space_required - space_remaining;
    let mut current_best_dir = "/".to_string();

    //Calculate parts 1 and 2
    for (dir_name, size) in &dir_sizes {
        if *size <= 100000 {
            part1 += *size;
        }
        if *size > shortfall && *size < *dir_sizes.get(&current_best_dir).unwrap() {
            current_best_dir = dir_name.clone();
        }
    }

    part2 = *dir_sizes.get(&current_best_dir).unwrap();

    (part1, part2)
}

// Recursive function calculates the size of the node
fn calculate_size(
    nodes: &HashMap<String, HashSet<String>>,
    sizes: &mut HashMap<String, usize>,
    dir_sizes: &mut HashMap<String, usize>,
    node: &str,
) -> usize {
    if sizes.contains_key(node) {
        *sizes.get(node).unwrap()
    } else if dir_sizes.contains_key(node) {
        *dir_sizes.get(node).unwrap()
    } else {
        //recurse
        let mut size: usize = 0;
        for child in nodes.get(node).unwrap().iter() {
            size += calculate_size(nodes, sizes, dir_sizes, child);
        }
        dir_sizes.insert(node.to_string(), size);
        size
    }
}

fn create_key(node: &str, current_directory: &str, parent_stack: &Vec<String>) -> String {
    let mut key = String::new();

    key += current_directory;

    for parent in parent_stack.iter().rev() {
        key += parent;
    }

    key += node;

    key
}

fn construct_filesystem(
    input: &Vec<String>,
    nodes: &mut HashMap<String, HashSet<String>>,
    sizes: &mut HashMap<String, usize>,
) {
    let re_cd_command = Regex::new(r"^\$ cd (?P<directory>\S+)$").unwrap();
    let re_ls_command = Regex::new(r"^\$ ls$").unwrap();
    let re_dir = Regex::new(r"^dir (?P<directory>\S+)$").unwrap();
    let re_file = Regex::new(r"^(?P<size>\d+) (?P<file>\S+)$").unwrap();

    let mut current_directory: String = "/".to_string();
    let mut parent_stack: Vec<String> = vec![];

    for line in input {
        if re_cd_command.is_match(line) {
            let directory = re_cd_command
                .captures(line)
                .unwrap()
                .name("directory")
                .unwrap()
                .as_str();
            if directory == "/" {
                //Go to root.
                current_directory = "/".to_string();
                let key = create_key(&current_directory, "", &parent_stack);
                if !nodes.contains_key(&key) {
                    // Not yet created the entry for this directory
                    nodes.insert(key.clone(), HashSet::new());
                }
            } else if directory == ".." {
                //Go up one level
                current_directory = parent_stack.pop().unwrap().to_string();
            } else {
                //Child of current directory
                parent_stack.push(current_directory.to_string());

                //Update the current directory to be the one we've just stepped into.
                current_directory = directory.to_string();
            }
        } else if re_ls_command.is_match(line) {
            //No op. The next lines will be output for the current directory
        } else if re_dir.is_match(line) {
            // Directory line
            let directory = re_dir
                .captures(line)
                .unwrap()
                .name("directory")
                .unwrap()
                .as_str();
            let key = create_key(directory, &current_directory, &parent_stack);
            if !nodes.contains_key(&key) {
                // Not yet created the entry for this directory
                nodes.insert(key.clone(), HashSet::new());
            }

            //We should be guaranteed to have a hash set for the current directory. This will panic otherwise, which is fine.
            nodes
                .get_mut(&create_key(&current_directory, "", &parent_stack))
                .unwrap()
                .insert(key);
        } else if re_file.is_match(line) {
            let file = re_file
                .captures(line)
                .unwrap()
                .name("file")
                .unwrap()
                .as_str();
            let size = re_file
                .captures(line)
                .unwrap()
                .name("size")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            let key = create_key(file, &current_directory, &parent_stack);
            //Add the file to the set of nodes under this directory
            nodes
                .get_mut(&create_key(&current_directory, "", &parent_stack))
                .unwrap()
                .insert(key.clone());

            //Add the size to the sizes map (doesn't matter if we've seen this before - just overwrite)
            sizes.insert(key.clone(), size);
        }
    }
}
