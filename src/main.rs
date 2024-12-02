use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    println!("Day 1, Puzzle 1");
    day_one_puzzle_one("inputs/input_1_1.txt");
    println!("Day 1, Puzzle 2");
    day_one_puzzle_two("inputs/input_1_1.txt");
}

fn read_input(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn str_to_int_tuple(line: &str) -> (i32, i32) {
    let string_pair: Vec<&str> = line.splitn(2, "   ").collect();
    return (
        string_pair[0].parse().unwrap(),
        string_pair[1].parse().unwrap()
    )
}

fn day_one_puzzle_one(file_path: &str) {
    // 1st December, Puzzle 1
    // Sum the difference beteween two sorted lists

    // Read file and map to two arrays
    let lines = read_input(file_path);
    // Map to two numeric vectors
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for line in lines {
        let (a_val, b_val) = str_to_int_tuple(&line);
        a.push(a_val);
        b.push(b_val)
    }
    // Sort
    a.sort();
    b.sort();
    let sorted_pairs = a.iter().zip(b.iter()).collect::<Vec<(&i32, &i32)>>();
    let diff_sum: i32 = sorted_pairs.into_iter().map(|x| (x.0 - x.1).abs()).sum();

    println!("{}", diff_sum)
}

fn day_one_puzzle_two(file_path: &str) {
    // 1st December, Puzzle 3
    // Sum the difference beteween two sorted lists based on occurences in list 2

    // Read file and map to two arrays
    let lines = read_input(file_path);
    // Map to two numeric vectors
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for line in lines {
        let (a_val, b_val) = str_to_int_tuple(&line);
        a.push(a_val);
        b.push(b_val)
    }
    // Sort
    a.sort();
    b.sort();

    // Count occurences of each value from a in b
    let mut count_map:HashMap<i32, usize> = HashMap::new();
    let a_uniq: HashSet<&i32> = HashSet::from_iter(a.iter());
    for &a_val in &a_uniq {
        let count_a = b.clone().into_iter().filter(|&n| n == *a_val).count();
        count_map.insert(*a_val, count_a);    
    }
    // Multiply a by times in b
    let mut s: usize = 0;
    for &a_val in &a_uniq {
        s = s + ((*a_val as usize) * count_map.get(a_val).unwrap());
    }
    println!("{}", s);
}