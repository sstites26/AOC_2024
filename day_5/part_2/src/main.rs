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
        let mut temp_string = String::from("");
        for c in l.chars() {
            temp_string.push(c);
        }

        let mut in_order = process_order(&temp_string, &mut rules_map);

        while !in_order {
            let resulting_vec = put_in_order(&temp_string, &mut rules_map);
            
            let mut index = 0;
            let size = resulting_vec.len();
            temp_string = String::from("");
            while index < size {
                temp_string.push_str(&resulting_vec[index].to_string());

                if index != size - 1 {
                    temp_string.push(',');
                }
                index += 1;
            }

            in_order = process_order(&temp_string, &mut rules_map);
            if in_order {
                //println!("THE STRING {} in order {}", l, temp_string);
                total += get_middle_num(&temp_string);
            }
        }
    }

    println!("{}", total);
}

fn put_in_order(line: &str, rules_map: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut parts: Vec<i32> = line.split(',').map(|x|->i32{x.parse().unwrap()}).collect();
    let new_rules = get_relevant_rules(&mut parts, rules_map);

    let mut clone_numbers = parts.clone();

    let mut index = 0;
    let last_index = parts.len();
    for p in &clone_numbers {
        if index != 0 {
            let current_index = index;
            let numbers_before_index = &clone_numbers[0..index];

            if new_rules.contains_key(p) {
                let mut rules_for_num: Vec<i32> = new_rules.get(p).unwrap().to_vec();

                for rule in rules_for_num {
                    if numbers_before_index.contains(&rule) {
                       let moving_index = numbers_before_index.iter().position(|&r| r == rule).unwrap();
                       parts = switch_elements(&mut parts, current_index.try_into().unwrap(), moving_index.try_into().unwrap()).to_vec();

                       return parts;
                    }
                }
            }
        }

        index += 1;
    }

    return parts;
}

fn switch_elements(report: &mut Vec<i32>, set_number_index: i32, moving_number_index: i32) -> &mut Vec<i32> {
    let moveVal = report[moving_number_index as usize];
    report.remove(moving_number_index as usize);

    if set_number_index == 0 {
        report.insert(0, moveVal);
    } else {
        report.insert((set_number_index) as usize, moveVal);
    }

    report
}

fn get_relevant_rules(report: &mut Vec<i32>, rules_map: &mut HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut new_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for (key, value) in rules_map.into_iter() {
        if report.contains(&key) {
            new_rules.insert(*key, value.to_vec());
        }
    }

    new_rules
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

    #[test]
    fn switch_elements_test() {
        let mut a = vec![1, 2, 3, 4, 5];
        
        a = switch_elements(&mut a, 0, 2).to_vec();
        assert_eq!(a, [3, 1, 2, 4, 5]);
        
        a = switch_elements(&mut a, 3, 4).to_vec();
        assert_eq!(a, [3, 1, 2, 5, 4]);
        
        a = switch_elements(&mut a, 1, 4).to_vec();
        assert_eq!(a, [3, 4, 1, 2, 5]);
        
        a = switch_elements(&mut a, 0, 4).to_vec();
        assert_eq!(a, [5, 3, 4, 1, 2]);
    }
}