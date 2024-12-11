use std::collections::HashMap;

//const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt");

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum ExpState {
    NotTested,
    Valid,
    NotValid
}

#[derive(Clone)]
#[derive(Debug)]
struct Exp {
    result: i64,
    nums: Vec<i64>,
    state: ExpState
}

fn main() {
    let mut expressions : Vec<Exp> = Vec::new();
    INPUT.lines().for_each(|line| {
        expressions.push(process_line(line));
    });

    let mut for_sure_not_valid = 0;
    let mut already_known_valid = 0;
    let mut max_operands: i64 = 0;
    for ex in &mut expressions {
        if ex.nums.len() > max_operands.try_into().unwrap() {
            max_operands = ex.nums.len().try_into().unwrap();
        }

        if ex.state == ExpState::Valid {
            already_known_valid += 1;
        }
    }

    let base: i64 = 3;
    max_operands -= 1;
    let max_perms = base.pow(max_operands.try_into().unwrap());

    let operand_vec = fill_vect_with_operands(max_perms, max_operands);

     let mut right_count = 0;
     let mut total:i64 = 0;
     for ex in expressions {
         let result:i64 = check_expression(ex.clone(), operand_vec.clone());
         total += result;

         if result != 0 {
            println!("Checking {:?}", ex);
            right_count += 1;
         } 
    }

     println!("{} {right_count}", total);
    // println!("TOTAL COUNT {}", INPUT.lines().count());
    // println!("POSSIBLE: {}", INPUT.lines().count() - (for_sure_not_valid + already_known_valid));
}

fn concat(first: i64, second: i64) -> i64 {
    let one = first.to_string();
    let two = second.to_string();

    let mut final_value = String::from(one);
    final_value.push_str(&two);

    final_value.parse::<i64>().expect("REASON")
}

fn check_expression(ex: Exp, operand_map: HashMap<i64, Vec<char>>) -> i64 {
    if ex.state != ExpState::NotValid {
        let base: i64 = 3;
        let perms = base.pow((ex.nums.len() - 1).try_into().unwrap());

        let mut index = 0;
         while index < perms {
            let ooop = operand_map.get(&index).unwrap();
            let counting = ex.nums.len() - 1;
            let diff = ((ooop.len() - counting) as i64).abs();
            let mut operand = ooop.clone();
            operand = (&operand[diff as usize..]).to_vec();

            let mut operand_index = 0;
            let mut total:i64 = 0;
            
            while operand_index < operand.len() {
                if operand_index == 0 {
                    let num1 = ex.nums[operand_index];
                    let num2 = ex.nums[operand_index + 1];
                    let c = operand.get(operand_index).unwrap();
                    if *c == '0' {
                        total = num1 + num2;
                    } else if *c == '1' {
                        total = num1 * num2;
                    } else if *c == '2' {
                        total = concat(num1, num2);
                    }
                } else {
                    let num1 = ex.nums[operand_index + 1];
                    let c = operand.get(operand_index).unwrap();
                    if *c == '0' {
                        let t = total;
                        total += num1;
                    } else if *c == '1' {
                        let t = total;
                        total *= num1;
                    } else if *c == '2' {
                        let t = total;
                        total = concat(total, num1);
                    }
                }

                operand_index += 1;
            }

            if total == ex.result {
                return total
            } 
            index += 1;
        }

        return 0
    }

    return 0
}

fn fill_vect_with_operands(num_permutations: i64, min_digits: i64) -> HashMap<i64, Vec<char>> {
    let mut operands: HashMap<i64, Vec<char>> = HashMap::new();

  	let mut index = 0;

    while index < num_permutations {
        let y = dec2bin(index);
        let mut char_vec: Vec<char> = y.to_string().chars().collect();

        char_vec = char_vec.iter().copied().rev().collect();
        char_vec.resize(min_digits.try_into().unwrap(), '0');
        char_vec = char_vec.iter().copied().rev().collect();
        
        operands.insert(index, char_vec);

        index += 1; 
    }

    operands
}

fn dec2bin(num:i64)->i64
{
    if num == 0
    {
        return 0;
    }
    else
    {
        return num % 3 + 10 * dec2bin(num / 3);
    }
}

fn process_line(line: &str) -> Exp {
    let expression: Vec<&str> = line.split(':').collect();
    let result: i64 = expression[0].parse().unwrap();

    let mut nums_str = expression[1];
    nums_str = &nums_str[1..nums_str.len()];
    
    let nums: &Vec<i64> = &nums_str.split(' ').map(|x|->i64{x.parse().unwrap()}).collect();
    
    return Exp { result: result, nums: nums.to_vec(), state: ExpState::NotTested };
}