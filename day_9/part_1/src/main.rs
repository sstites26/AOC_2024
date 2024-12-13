const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let line = INPUT.lines().nth(0).expect("bad index");

    let info_size = 2;
    let mut set_index = 0;

    let first_info = &line[0..info_size];
    let last_info = &line[line.len() - set_index - 1..];

    let set_pairs_left = (line.len() - 3) / 4;
    let mut sets_pairs_parsed = 0;

    let mut first_info_index = set_index;
    let mut last_info_index = line.len() / 2;

    let very_first_pair = get_digits((*first_info).parse::<i64>().unwrap());
    

    let very_first_str = get_string(very_first_pair.0, very_first_pair.1, first_info_index as i64);
    let very_second_str = get_string((*last_info).parse::<i64>().unwrap(), 0, last_info_index as i64);
    
    let r1 = process_string(very_first_str.to_vec(), very_second_str.to_vec());

    set_index += 1;
    first_info_index += 1;
    last_info_index -= 1;

    let mut beginning = r1.0;
    let mut ending = r1.1;


    let mut test_index = 0;
    while sets_pairs_parsed < set_pairs_left {
        let pairs = get_next_pair(line, set_index.try_into().unwrap());

        let first_pair = get_digits(pairs.0);
        let second_pair = get_digits(pairs.1);

        let mut first_str = get_string(first_pair.0, first_pair.1, first_info_index as i64);
        let mut second_str = get_string(second_pair.0, second_pair.1, last_info_index as i64);

        beginning.append(&mut first_str);
        second_str.append(&mut ending);
        ending = second_str;

        let result = process_string(beginning.to_vec(), ending.to_vec());
        
        beginning = result.0;
        ending = result.1;

        sets_pairs_parsed += 1;
        set_index += 1;
        first_info_index += 1;
        last_info_index -= 1;

        test_index += 1;
    }

    let mut final_end = sort_my_string(ending.clone());

    beginning.append(&mut final_end);
    let final_final = condense_final_string(beginning.clone());

    let mut total = 0;

    let mut value_index = 0;
    for value in final_final {
        if value != -1 {
            let temp = value * value_index;
            total += temp;
        }
        value_index += 1;
    }

     println!("{}", total);
}

fn sort_my_string(mut string_to_sort: Vec<i64>) -> Vec<i64> {
    let mut front_index = 0;
    let mut back_index = string_to_sort.len() - 1;
    // println!("{:?}", string_to_sort);

    while front_index < string_to_sort.len() {
        let mut back_value = string_to_sort[back_index];

        // println!("Front Index {} Back Index {} Front Value {} Back Value {}", front_index, back_index, string_to_sort[front_index], back_value);
        if string_to_sort[front_index] == -1 {
            while back_index > 0 && back_value == -1 {
                // println!("1. Back Index {} Back {}", back_index, back_value);
                back_index -= 1;
                back_value = string_to_sort[back_index];
                // println!("2. Back Index {} Back {}", back_index, back_value);

                if back_index <= front_index {
                    break;
                }
            }
                        
            let _ = std::mem::replace(&mut string_to_sort[front_index], back_value);
            let _ = std::mem::replace(&mut string_to_sort[back_index], -1);

            // println!("SWAPPED: {:?}", string_to_sort);
        }

        front_index += 1;

        if front_index >= back_index {
            break;
        }
    }

    string_to_sort
}

fn condense_final_string(mut string_to_condense: Vec<i64>) -> Vec<i64> {
    let mut last_index = string_to_condense.len() - 1;
    let mut beginning_index = 0;

    while beginning_index < string_to_condense.len() {
         if string_to_condense[beginning_index] == -1 {
            let c2 = string_to_condense[last_index];
            
            let _ = std::mem::replace(&mut string_to_condense[beginning_index], c2);
            let _ = std::mem::replace(&mut string_to_condense[last_index], -1);

            last_index -= 1;
        }
        beginning_index += 1;

        if beginning_index > last_index {
            break;
        }
    }

    string_to_condense
}

fn process_string(mut first: Vec<i64>, mut second: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut open_spots : Vec<i64> = Vec::new();
    
    let mut index = 0;
    for spot in &first {
        if *spot == -1 {
            open_spots.push(index);
        }
        index += 1;
    }

    let second_len = second.len();
    index = 0;
    let mut added = 0;
    while (index as i64) < open_spots.len().try_into().unwrap() {
        let mut second_index = 0;

        while second_index < second_len {
            let c = second.pop();
            if c != Some(-1) && c != None {
                first[open_spots[index as usize] as usize] = c.unwrap(); 
                added += 1;
                break;
            }

            if added == open_spots.len() {
                break;
            }
            second_index += 1;
        }

        index += 1;
    }

    (first, second)
}

fn get_string(first_num: i64, second_num: i64, value: i64) -> Vec<i64> {
    let mut final_str : Vec<i64> = Vec::new();

    let mut index = 0;
    while index < first_num {
        // print!("{} ", value);
        let c = value.try_into().unwrap();
        final_str.push(c);
        index += 1;
    }

    index = 0;
    while index < second_num {
        final_str.push(-1);
        index += 1;
    }

    final_str
}

fn get_digits(number: i64) -> (i64, i64) {
    let first = number / 10;
    let second = number % 10;

    (first, second)
}

fn get_next_pair(line: &str, index: i64) -> (i64, i64) { 
    let info_size = 2 as usize;
    let new_index = index as usize;
    let first_start = (new_index * info_size) as usize;
    let second_start = (line.len() - (new_index * 2) - 1) as usize;

    let first_info = &line[first_start..first_start + info_size];
    let last_info = &line[second_start..second_start + info_size];

    ((*first_info).parse::<i64>().unwrap(), (*last_info).parse::<i64>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_my_string_test() {
        
        let mut first = [2 , 2, 2, 2, -1, -1, 5, 7, -1, 9];
        let result = sort_my_string(first.to_vec());
        assert_eq!(result, [2 , 2, 2, 2, 9, 7, 5, -1, -1, -1]);
    }
}