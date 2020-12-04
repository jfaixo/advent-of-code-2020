use std::{env, fs};
use std::process::exit;


#[derive(Debug, Clone, PartialEq)]
enum SquareContent {
    EMPTY,
    TREE
}

#[derive(Debug)]
struct TreeMap {
    map: Vec<SquareContent>,
    width: usize,
    height: usize,
}

impl TreeMap {
    fn set_tree(&mut self, x: usize, y: usize) {
        self.map[y * self.width + (x % self.width)] = SquareContent::TREE;
    }

    fn get(&self, x: usize, y: usize) -> SquareContent {
        return self.map[y * self.width + (x % self.width)].clone();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let tree_map = parse_text_file(args[1].clone());

    eprintln!("{:?}", tree_map);

    println!("part 1: {}", get_tree_number_in_path_part_1(&tree_map));
    println!("part 2: {}", get_tree_number_in_path_part_2(&tree_map));
}

fn parse_text_file(file_name: String) -> TreeMap {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    let raw_entries : Vec<&str> = file_content
        .split_ascii_whitespace()
        .collect();
    let height = raw_entries.len();
    let width = raw_entries[0].len();
    let map = vec![SquareContent::EMPTY; width * height];
    let mut tree_map = TreeMap { map, width, height };

    for y in 0..height {
        let mut x : usize = 0;
        raw_entries[y].chars().for_each(|c| {
            if c == '#' {
                tree_map.set_tree(x, y);
            }
            x += 1;
        });
    }

    return tree_map;
}

fn get_tree_number_in_path(tree_map: &TreeMap, slope_x: usize, slope_y: usize) -> u64 {
    let mut tree_count : u64 = 0;

    let mut current_x : usize = 0;
    let mut current_y : usize = 0;

    while current_y < tree_map.height {
        if tree_map.get(current_x, current_y) == SquareContent::TREE {
            tree_count += 1;
        }

        current_x += slope_x;
        current_y += slope_y;
    }

    return tree_count;
}

fn get_tree_number_in_path_part_1(tree_map: &TreeMap) -> u64 {
    return get_tree_number_in_path(tree_map, 3, 1);
}

fn get_tree_number_in_path_part_2(tree_map: &TreeMap) -> u64 {
    let slope_1_1 = get_tree_number_in_path(tree_map, 1, 1);
    let slope_3_1 = get_tree_number_in_path(tree_map, 3, 1);
    let slope_5_1 = get_tree_number_in_path(tree_map, 5, 1);
    let slope_7_1 = get_tree_number_in_path(tree_map, 7, 1);
    let slope_1_2 = get_tree_number_in_path(tree_map, 1, 2);

    return slope_1_1 * slope_3_1 * slope_5_1 * slope_7_1 * slope_1_2;
}
