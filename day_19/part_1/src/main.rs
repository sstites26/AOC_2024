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

    // let mut arg_str = "r|wr|b|g|bwu|rb|gb|br";
    // let re = Regex::new(format!(r#"{}"#, arg_str).as_str()).unwrap();.
    let re = Regex::new(format!(r#"^({})+$"#, arg_str).as_str()).unwrap();
    let s: String = re.replace_all(&temp_string, "").into();

    s
}

fn process_parts(line: &str) -> Vec<&str> {
    let mut size_map: HashMap<i32, HashSet<&str>> = HashMap::new();
    let parts: Vec<&str> = line.split(',').map(|l| l.trim()).collect();
    
    parts

    // let mut min = 1000;
    // let mut max = 0;

    // for p in &parts {
    //     let len = p.len() as i32;

    //     if len > max {
    //         max = len;
    //     }

    //     if len < min {
    //         min = len;
    //     }

    //     if size_map.contains_key(&len) {
    //         let part_vec = size_map.get_mut(&len);
    //         part_vec.expect("REASON").insert(p);
    //     } else {
    //         let mut parts : HashSet<&str> = HashSet::new();
    //         parts.insert(p);

    //         size_map.insert(len.try_into().unwrap(), parts);
    //     }
    // }

    // let mut index = 0;
    // let v = vec! [3, 2, 1];

    // let mut final_parts : Vec<&str> = Vec::new();
    // while index < v.len() {
    //     let x = v[index];
    //     let parts = size_map.get(&x).expect("REASON");

    //     for p in parts {
    //         final_parts.push(p);
    //     }

    //     index += 1;
    // }

    // final_parts
}