
use regex::Regex;

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut total = 0;
    let mut evaluate = true;
    for line in INPUT.lines() {
        let return_val = process_line(line, evaluate);
        total = total + return_val.0;
        evaluate = return_val.1;
    }
    println!("{}", total);
}

fn process_line(line: &str, eval: bool) -> (i64, bool) {
    let re = Regex::new(r"mul\(\d*,\d*\)|don't()|do()").unwrap();
    let matches = re.captures_iter(line);

    let mut evaluate = eval;
    let mut total = 0;
    for expression in matches {
        let express_string = expression.get(0).unwrap().as_str();
        if express_string == "don't" {
            evaluate = false;
        } else if express_string == "do" {
            evaluate = true;
        } else {
            if evaluate {
                total = total + process_expression(expression.get(0).unwrap().as_str());
            }
        }
    }

    (total, evaluate)
}

fn process_expression(expression: &str) -> i64 {
    let comma = expression.chars().position(|c| c == ',').unwrap();
    let first_num = &expression[4..comma];
    let second_num = &expression[comma + 1..expression.len() - 1];
    
    let value = first_num.parse::<i64>().unwrap() * second_num.parse::<i64>().unwrap();
    //println!("{} - {} x {} = {}", expression, first_num, second_num, value);

    value
}