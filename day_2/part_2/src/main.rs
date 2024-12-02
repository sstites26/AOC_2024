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

    let unsafe_count = is_line_safe(&numbers, increasing);

    if unsafe_count == 0 {
        return true;
    }

    if unsafe_count > 0 {
        let mut index_to_remove = 0;
        
        while index_to_remove < (numbers.len() as i32) {
            if try_modified(&numbers, increasing, index_to_remove) {
                return true
            }  
            index_to_remove = index_to_remove + 1;
        }
    }
    
    return false
}

fn try_modified(arr: &[i32], increasing: bool, index_to_remove: i32) -> bool {
    let mut nums : Vec<i32> = Vec::new();
    
    let mut index = 0;
    for number in arr {
        if index != index_to_remove {
            nums.push(*number);
        }

        index = index + 1;
    }

    let is_it_safe = is_line_safe(&nums, increasing);
    if is_it_safe == 0 {
        return true
    }

    return false
}

fn is_line_safe(arr: &[i32], increasing: bool) -> i32 {
    let mut count_unsafe = 0;
    let mut peekable_arr = arr.iter().peekable();
    while let Some(l) = peekable_arr.next() {
        if let Some(next_element) = peekable_arr.peek() {
            let current_value :i32 = *l;
            let next_value :i32 = **next_element;

            let diff = (current_value - next_value).abs();

            if (diff <= 0 || diff >= 4) || (increasing && current_value > next_value) || 
                    (!increasing && current_value < next_value) {
                count_unsafe = count_unsafe + 1;
            }
        }
    }
    
    return count_unsafe
}

fn get_values(line: &str) -> Vec<i32> {
    let nums: Vec<i32> = line.trim()
                             .split_whitespace()
                             .map(i32::from_str)
                             .map(Result::unwrap)
                             .collect();
    nums
}