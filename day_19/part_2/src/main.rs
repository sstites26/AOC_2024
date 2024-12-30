use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools; // 0.8.2

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let mut index = 0;
    let mut puzzles : Vec<&str> = Vec::new();
    let mut parts : Vec<&str> = Vec::new();

    for line in INPUT.lines() {
        if index == 0 {
            parts = process_parts(line);
        } else {
            if line != "" {
                puzzles.push(line);
            }
        }

        index += 1;
    }

    let mut total = 0;

    for puzzle in puzzles {
        let answer = solve_puzzle(puzzle.to_string(), parts.clone());

        if answer == "" {
            // println!("GOOD: {}", puzzle);
            total += 1;
        } else {
            // println!("BAD: {}", puzzle);
        }
    }

    println!("{}", total);
}

fn solve_puzzle(puzzle: String, parts: Vec<&str>) -> String {
    let mut temp_string = puzzle.clone();

    let mut arg_str : String = "".to_string();

    for p in parts {
        arg_str = format!("{}{}", arg_str, p);
        arg_str = format!("{}{}", arg_str, "|");
    }

    arg_str = (&arg_str[0..arg_str.len() - 1]).to_string();
    
    let re = Regex::new(format!(r#"^({})+$"#, arg_str).as_str()).unwrap();
    let matches: Vec<_> = re.find_iter(&temp_string).map(|m| m.as_str()).collect();

    println!("{:?}", matches);

    // let re2 = Regex::new(r"\b\w{13}\b").unwrap();
    // let hay = "Retroactively relinquishing remunerations is reprehensible.";
    // let matches2: Vec<_> = re2.find_iter(hay).map(|m| m.as_str()).collect();
    // println!("{:?}", matches2);

    let s: String = re.replace_all(&temp_string, "").into();

    s
}

fn process_parts(line: &str) -> Vec<&str> {
    let mut size_map: HashMap<i32, HashSet<&str>> = HashMap::new();
    let parts: Vec<&str> = line.split(',').map(|l| l.trim()).collect();
    
    parts
}