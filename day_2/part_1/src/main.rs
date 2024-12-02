use std::str::FromStr;

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut count = 0;
    for line in INPUT.lines() {
        let good = process_line(line);

        if good {
            count = count + 1
        }
    }

    println!("{}", count);
}

fn process_line(line: &str) -> bool {
    let numbers = get_values(line); 

    let mut increasing = true;
    if numbers[0] > numbers[numbers.len() - 1] {
        increasing = false;
    }

    let is_good = is_line_safe(&numbers, increasing);
    
    is_good
}

fn is_line_safe(arr: &[i32], increasing: bool) -> bool {
    let mut peekable_arr = arr.iter().peekable();
    while let Some(l) = peekable_arr.next() {
        if let Some(next_element) = peekable_arr.peek() {
            let current_value :i32 = *l;
            let next_value :i32 = **next_element;

            let diff = (current_value - next_value).abs();

            if (diff <= 0 || diff >= 4) || (increasing && current_value > next_value) || 
                    (!increasing && current_value < next_value) {
                return false;
            }
        }
    }
    
    return true
}

fn get_values(line: &str) -> Vec<i32> {
    let nums: Vec<i32> = line.trim()
                             .split_whitespace()
                             .map(i32::from_str)
                             .map(Result::unwrap)
                             .collect();
    nums
}