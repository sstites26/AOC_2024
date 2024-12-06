use std::collections::HashMap;

const INPUT_RULES: &str = include_str!("test_input_rules.txt");
const INPUT_ORDERING: &str = include_str!("test_input_ordering.txt");
//const INPUT_RULES: &str = include_str!("real_input_rules.txt"); 
//const INPUT_ORDERING: &str = include_str!("real_input_ordering.txt");

fn main() {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    INPUT_RULES.lines().for_each(|line| process_rules(line, &mut rules_map)); 

    let mut total = 0;
    for l in INPUT_ORDERING.lines() {
        let in_order = process_order(l, &mut rules_map);

        if in_order {
            let x = get_middle_num(l);
            total += x;
        }
    }

    println!("{}", total);
}

fn process_order(line: &str, rules_map: &mut HashMap<i32, Vec<i32>>) -> bool {
    let parts: Vec<i32> = line.split(',').map(|x|->i32{x.parse().unwrap()}).collect();

    let mut index = 0;
    for p in &parts {
        if rules_map.contains_key(&p) {
            let n = rules_map.get_mut(&p).expect("DOESNT EXIST");
            for ni in n {
                if index != 0 {
                    let before = is_number_before_index(&parts, *ni, index);
                    if before {
                        return false;
                    }
                }
            }
        }

        index += 1;
    }
    
    return true;
}

fn get_middle_num(line: &str) -> i32 {
    let parts: Vec<i32> = line.split(',').map(|x|->i32{x.parse().unwrap()}).collect();

    let count = parts.len();
    let half = count / 2;

    parts[half]
}

fn is_number_before_index(numbers: &Vec<i32>, value: i32, end_index: usize) -> bool {
    let first_part =  &numbers[0..end_index]; 

    for n in first_part {
        if *n == value {
            return true;
        }
    }

    return false;
}

fn process_rules(line: &str, rules_map: &mut HashMap<i32, Vec<i32>>) {
    let parts = get_values(line, '|');
    if rules_map.get(&parts.0) == None {
        let mut v : Vec<i32> = Vec::new();
        v.push(parts.1);

        rules_map.insert(parts.0, v);        
    } else {
       rules_map.get_mut(&parts.0).expect("DOESNT EXIST").push(parts.1);
    }
}

fn get_values(line: &str, delimiter: char) -> (i32, i32) {
    let parts: Vec<&str> = line.split(delimiter).collect();
    (
        parts[0].parse().expect("Failed to parse first integer"),
        parts[1].parse().expect("Failed to parse second integer"),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_number_before_index_test() {
        let mut a = vec![1, 2, 3, 4, 5];
        
        let mut good = is_number_before_index(&a, 3, 3);
        assert_eq!(good, true);

        good = is_number_before_index(&a, 5, 3);
        assert_eq!(good, false);

        good = is_number_before_index(&a, 1, 4);
        assert_eq!(good, true);

        a = vec![ 75,97,47,61,53];
        good = is_number_before_index(&a, 75, 1);
        assert_eq!(good, true);
    }
}