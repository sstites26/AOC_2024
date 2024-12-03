
use regex::Regex;

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut total = 0;
    for line in INPUT.lines() {
        total = total + process_line(line);
    }
    println!("{}", total);
}

fn process_line(line: &str) -> i64 {
    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let matches = re.captures_iter(line);

    let mut total = 0;
    for expression in matches {
        total = total + process_expression(expression.get(0).unwrap().as_str());
    }

    total
}

fn process_expression(expression: &str) -> i64 {
    let comma = expression.chars().position(|c| c == ',').unwrap();
    let first_num = &expression[4..comma];
    let second_num = &expression[comma + 1..expression.len() - 1];
    
    let value = first_num.parse::<i64>().unwrap() * second_num.parse::<i64>().unwrap();
    //println!("{} - {} x {} = {}", expression, first_num, second_num, value);

    value
}