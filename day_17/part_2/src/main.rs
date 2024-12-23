use std::collections::HashSet;

// const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Instruction {
    opcode: i64,
    operand: i64
}

fn main() {
    let a_line = INPUT.lines().nth(0).expect("REASON");
    let b_line = INPUT.lines().nth(1).expect("REASON");
    let c_line = INPUT.lines().nth(2).expect("REASON");
    let instruction_line = INPUT.lines().nth(4).expect("REASON");
    
    let mut a : i64 = 0;
    let mut b : i64 = 0;
    let mut c : i64 = 0;

    parse_registers(a_line, b_line, c_line, &mut a, &mut b, &mut c);

    let mut original_instructions = Vec::new(); 

    let trimmed = &instruction_line[9..instruction_line.len()];
    for part in trimmed.split(",") {
        if let Ok(num) = part.parse::<i64>() {
            original_instructions.push(num);
        }
    }
    
    let instructions = parse_instructions(instruction_line);

    let mut outputs : Vec<i64> = Vec::new();

    // println!("A: {} B: {} C: {}", a, b, c);

    let mut found = false;
    
    // a = 36899064063712;
    // a = 35899064063712;
    //a = 40000000000000;
    // a = 60000004455015;
    
    // a = 60000095022693;
    // a = 68457095022693;
    // stopped at A:68459246843511

    let orig_a = a;
    let mut counting = 0;

    let mut a_checked : HashSet<i64> = HashSet::new();

    while !found {
        let mut index : i64 = 0;
        a = orig_a + counting;
        // a += 5;

        // while a <= orig_a || a_checked.contains(&a) {
        //     a += 5;
        // }
        // a_checked.insert(a);

        println!("A:{}", a);
        while index < instructions.len().try_into().unwrap() {
            let instruction = instructions[index as usize];
            let result = process_instruction(instruction, &mut a, &mut b, &mut c, &mut outputs);

            if outputs.len() > instructions.len() * 2 {
                break;
            } else {
                if outputs.len() == instructions.len() * 2 {
                    let matching = outputs.iter().zip(&original_instructions).filter(|&(outputs, original_instructions)| outputs == original_instructions).count();
                
                    if matching == original_instructions.len() {
                        println!("YEA!! {} {:?}", a, outputs);
                        found = true;
                        break;
                    }
                }
            }

            if result != -1 {
                index = result;
            } else {
                index += 1;
            }
        }
        println!("A: {} {:?}", a, outputs);
        counting += 1;
        outputs.clear();
    }
}

fn process_instruction(instruction: Instruction, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64, outputs: &mut Vec<i64>) -> i64 {
    if instruction.opcode == 0 {
        adv(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 1 {
        bxl(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 2 {
        bst(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 3 {
        return jnz(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 4 {
        bxc(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 5 {
        out(instruction.operand, register_a, register_b, register_c, outputs);
    } else if instruction.opcode == 6 {
        bdv(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 7 {
        cdv(instruction.operand, register_a, register_b, register_c);
    }

    return -1;
}

fn adv(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }
    
    let base: i64 = 2;
    let denom = base.pow(value.try_into().unwrap());
    let result = *register_a / denom;

    // println!("A Dividing A:{} by 2^{} = {}\n", register_a, value, result);

    *register_a = result;
}

fn bxl(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let result = *register_b ^ operand;
    // println!("Bitwise Oring B:{} and {} = {}\n", register_b, operand, result);
    *register_b = result;
}

fn bst(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }

    let result = value % 8;
    // println!("Modulo 8: {} % 8 = {}", value, result);
    *register_b = result;
}

fn jnz(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) -> i64 {
    // println!("Jumping to {}", operand);

    if *register_a == 0 {
        return -1;
    } else {
        return operand;
    }
}

fn bxc(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let result = *register_b ^ *register_c;
    // println!("Bitwise Oring B:{} and C:{} = {}\n", register_b, register_c, result);
    *register_b = result;
}

fn out(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64, outputs: &mut Vec<i64>) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }

    let result = value % 8;
    // println!("THE OUTPUT IS {}", result);
    outputs.push(result);
}

fn bdv(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }
    
    let base: i64 = 2;
    let denom = base.pow(value.try_into().unwrap());
    let result = *register_a / denom;

    // println!("B Dividing A:{} by 2^{} = {}\n", register_a, value, result);

    *register_b = result;
}

fn cdv(operand: i64, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }
    
    let base: i64 = 2;
    let denom = base.pow(value.try_into().unwrap());
    let result = *register_a / denom;

    // println!("C Dividing A:{} by 2^{} = {}\n", register_a, value, result);

    *register_c = result;
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let trimmed = &line[9..line.len()];
    let parts: Vec<&str> = trimmed.split(",").collect();
    let mut instructions : Vec<Instruction> = Vec::new();

    let mut index = 0;

    while index < parts.len() {
        let first = parts[index].parse().unwrap();
        let second = parts[index + 1].parse().unwrap();

        let instruction = Instruction{opcode: first, operand: second};
        instructions.push(instruction);

        index += 2;
    }

    instructions
}

fn parse_registers(a_str: &str, b_str: &str, c_str: &str, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) {
    let a_index = a_str.find(": ").unwrap() + 1;
    let b_index = b_str.find(": ").unwrap() + 1;
    let c_index = c_str.find(": ").unwrap() + 1;

    let a = (&a_str[a_index + 1..a_str.len()]).parse().unwrap();
    let b = (&b_str[b_index + 1..b_str.len()]).parse().unwrap();
    let c = (&c_str[c_index + 1..c_str.len()]).parse().unwrap();

    *register_a = a;
    *register_b = b;
    *register_c = c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions() {
        let mut a = 0;
        let mut b = 0;
        let mut c = 9;

        process_instruction(Instruction{opcode:2, operand:6}, &mut a, &mut b, &mut c);
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 9);

        a = 0;
        b = 29;
        c = 0;
        process_instruction(Instruction{opcode:1, operand:7}, &mut a, &mut b, &mut c);
        assert_eq!(a, 0);
        assert_eq!(b, 26);
        assert_eq!(c, 0);

        a = 0;
        b = 2024;
        c = 43690;
        process_instruction(Instruction{opcode:4, operand:0}, &mut a, &mut b, &mut c);
        assert_eq!(a, 0);
        assert_eq!(b, 44354);
        assert_eq!(c, 43690);
    }
}