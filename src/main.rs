use std::{collections::{HashMap, HashSet}, fs::{self}};

use regex::Regex;

fn main() {
    println!("Day 1, Puzzle 1");
    day_one_puzzle_one("inputs/input_1_1.txt");
    println!("Day 1, Puzzle 2");
    day_one_puzzle_two("inputs/input_1_1.txt");
    println!("Day 2, Puzzle 1");
    day_two_puzzle_one("inputs/input_2_1.txt");
    println!("Day 2, Puzzle 2");
    day_two_puzzle_two("inputs/input_2_1.txt");
    println!("Day 3, Puzzle 1");
    day_three_puzzle_one("inputs/input_3_1.txt");
    println!("Day 3, Puzzle 2");
    day_three_puzzle_two("inputs/input_3_1.txt");
    println!("Day 4, Puzzle 1");
    day_four_puzzle_one("inputs/input_4_1.txt");
    println!("Day 4, Puzzle 2");
    day_four_puzzle_two("inputs/input_4_1.txt");
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
    // 1st December, Puzzle 2
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

fn report_valid(num_vec: &Vec<i32>) -> bool {
    // Determine if a report is valid for day 2

    let diff: Vec<i32> = num_vec
        .iter().zip(num_vec.iter().skip(1))
        .map(|(n, np)| np - n).collect();
    // Determine if all differences have the same sign
    if !(diff.iter().all(|x| *x > 0) | diff.iter().all(|x| *x < 0)) {
        return false
    }
    // Determine if all absolute differences in the required range
    if !diff.iter().all(|x| (1 <= x.abs()) && (x.abs() <= 3)) {
        return false
    }
    true
}

fn day_two_puzzle_one(file_path: &str) {
    // 2nd December, Puzzle 1
    // Reports are safe if: monotonically decreasing or increasing
    // and differ by between 1 and 3

    let lines: Vec<String> = read_input(file_path);
    // Convert lines to vectors of numbers
    let mut valid_count: u32 = 0;
    for line in lines {
        let numbers: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let valid: bool = report_valid(&numbers);
        if valid {
            valid_count += 1;
        }
    }
    println!("{valid_count}");
}

fn day_two_puzzle_two(file_path: &str) {
    // 2nd December, Puzzle 2
    // Safe if can remove one level and pass checks
    let lines: Vec<String> = read_input(file_path);
    // Convert lines to vectors of numbers
    let mut valid_count: u32 = 0;
    for line in lines {
        // Test valid with no alterations
        let numbers: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let valid: bool = report_valid(&numbers);
        if valid {
            valid_count += 1;
            continue;
        }
        // Test if vector with any element removed is valid
        for i in 0..numbers.len() {
            let missing = numbers.iter()
                .enumerate()
                .filter(|x| x.0 != i)
                .map(|x| *x.1)
                .collect();
            if report_valid(&missing) {
                valid_count += 1;
                break;
            }
        }
    }
    println!("{valid_count}");
}

fn day_three_puzzle_one(file_path: &str) {
    // 3rd December, Puzzle 1
    // Perform some multiplications in messy text

    let lines = read_input(file_path);
    let mut running_sum: usize = 0;
    for line in lines {
        let re = Regex::new(
            r"mul\((\d*),(\d*)\)"
        ).unwrap();
        for (_, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
            let an: usize = a.parse().unwrap();
            let bn: usize = b.parse().unwrap();
            running_sum += an * bn;
        }
    }
    println!("{running_sum}");
}

fn day_three_puzzle_two(file_path: &str) {
    // 3rd December, Puzzle 2
    // Perform some multiplications, but not others!

    let lines = read_input(file_path);
    let mut running_sum: usize = 0;
    let mut activated: bool = true;
    for line in lines {
        let re = Regex::new(
            r"mul\((\d*),(\d*)\)|do(?:n't)?\(\)"
        ).unwrap();
        for c in re.captures_iter(&line) {
            // Determine if the number groups were matched
            let a = c.get(1);
            match a {
                None => {
                    let command = c.get(0).unwrap();
                    match command.as_str() {
                        "do()" => activated = true,
                        "don't()" => activated = false,
                        &_ => {}
                    }
                },
                Some(am) => {
                    let an: usize = am.as_str().parse().unwrap();
                    let bn: usize = c.get(2).unwrap().as_str().parse().unwrap();
                    running_sum += an * bn * (if activated {1} else {0});
                }
            }
        } 
    }
    println!("{running_sum}");
}

fn count_word(line: &str, find: &str) -> usize {
    let forward = line.matches(find).count();
    let rev_find: String = find.as_bytes().iter().rev().map(|x| *x as char).collect();
    let reverse = line.matches(&rev_find).count();
    forward + reverse
}

fn extract_col(wordsearch: &Vec<String>, j: usize) -> String {
    // Get a column from the wordsearch as a single string
    wordsearch.iter().map(|x| x.as_bytes()[j] as char).collect()
}

fn extract_diagonal(wordsearch: &Vec<String>, k: usize, main: bool) -> String {
    // For an n x m matrix there are m+n-1 diagonals
    // The main diagonal is in the sequence a_ij, a_i-1,j+1
    // The antidigonal is the sequence a_ij, a_i+1,_j+1

    // Determine a starting location for the k-th diagonal
    let (m, n) = (wordsearch.len(), wordsearch[0].len());
    let (mut i, mut j) = (
        if k > m - 1 {
            m - 1
        } else {
            k
        }, 
        if main {
            if k > m - 1 {k - n + 1} else {0}
        } else {
            if k > m - 1 {m + n - k - 2} else {n - 1}
        }
    );
    let mut line: Vec<char> = Vec::new();
    let j_term = if main {n - 1} else {0};
    loop {
        line.push(wordsearch[i].as_bytes()[j] as char);
        if (i == 0) | (j == j_term) {
            break;
        }
        i = i - 1;
        j = if main {j + 1} else {j - 1};
    }
    let temp: String = line.iter().collect();
    return line.iter().collect();
}

fn day_four_puzzle_one(file_path: &str) {
    // 4th December, Puzzle 1
    // Wordsearch
    // Assuming we only have ASCII input to make string operations simpler

    let wordsearch: Vec<String> = read_input(file_path);
    let mut count: usize = 0;
    let needle = "XMAS";
    for row in &wordsearch {
        count += count_word(&row, needle);
    }
    for col in 0..wordsearch[0].len() {
        count += count_word(&extract_col(&wordsearch, col), needle);
    }
    for diag in 0..(wordsearch.len() + wordsearch[0].len() - 1) {
        count += count_word(&extract_diagonal(&wordsearch, diag, true), needle);
        count += count_word(&extract_diagonal(&wordsearch, diag, false), needle);
    }
    println!("{count}")
}

fn extract_submat(wordsearch: &Vec<String>, i: usize, j: usize) -> Vec<String> {
    // Get a 3x3 submatrix of the wordsearch
    wordsearch[i..i+3].iter().map(|x| {
        let t: String = x.as_bytes()[j..j+3].iter().map(|x| *x as char).collect();
        t
    }).collect()
}

fn day_four_puzzle_two(file_path: &str) {
    let wordsearch: Vec<String> = read_input(file_path);
    let needle: &str = "MAS";
    let mut count: usize = 0;
    for i in 0..wordsearch.len() - 2 {
        for j in 0..wordsearch[0].len() - 2 {
            let submat = extract_submat(&wordsearch, i, j);
            let diag = extract_diagonal(&submat, 2, true);
            let anti = extract_diagonal(&submat, 2, false);
            let hit: bool = (count_word(&diag, &needle) == 1) & (count_word(&anti, &needle) == 1);
            count += hit as usize;
        }
    }
    println!("{count}");
}