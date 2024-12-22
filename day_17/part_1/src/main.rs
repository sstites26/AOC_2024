const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Instruction {
    opcode: i32,
    operand: i32
}

fn main() {
    let a_line = INPUT.lines().nth(0).expect("REASON");
    let b_line = INPUT.lines().nth(1).expect("REASON");
    let c_line = INPUT.lines().nth(2).expect("REASON");
    let instruction_line = INPUT.lines().nth(4).expect("REASON");
    
    let mut a : i32 = 0;
    let mut b : i32 = 0;
    let mut c : i32 = 0;

    parse_registers(a_line, b_line, c_line, &mut a, &mut b, &mut c);
    let instructions = parse_instructions(instruction_line);

    // println!("A: {} B: {} C: {}", a, b, c);

    let mut index : i32 = 0;

    while index < instructions.len().try_into().unwrap() {
        let instruction = instructions[index as usize];

        let result = process_instruction(instruction, &mut a, &mut b, &mut c);

        if result != -1 {
            index = result;
        } else {
            index += 1;
        }
        // println!("A: {} B: {} C: {}", a, b, c);
    }
}

fn process_instruction(instruction: Instruction, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) -> i32 {
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
        out(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 6 {
        bdv(instruction.operand, register_a, register_b, register_c);
    } else if instruction.opcode == 7 {
        cdv(instruction.operand, register_a, register_b, register_c);
    }

    return -1;
}

fn adv(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }
    
    let base: i32 = 2;
    let denom = base.pow(value.try_into().unwrap());
    let result = *register_a / denom;

    // println!("A Dividing A:{} by 2^{} = {}\n", register_a, value, result);

    *register_a = result;
}

fn bxl(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
    let result = *register_b ^ operand;
    // println!("Bitwise Oring B:{} and {} = {}\n", register_b, operand, result);
    *register_b = result;
}

fn bst(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
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

fn jnz(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) -> i32 {
    // println!("Jumping to {}", operand);

    if *register_a == 0 {
        return -1;
    } else {
        return operand;
    }
}

fn bxc(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
    let result = *register_b ^ *register_c;
    // println!("Bitwise Oring B:{} and C:{} = {}\n", register_b, register_c, result);
    *register_b = result;
}

fn out(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
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
    println!("OUT: {}", result);
}

fn bdv(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }
    
    let base: i32 = 2;
    let denom = base.pow(value.try_into().unwrap());
    let result = *register_a / denom;

    // println!("B Dividing A:{} by 2^{} = {}\n", register_a, value, result);

    *register_b = result;
}

fn cdv(operand: i32, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
    let mut value = operand;

    if operand == 4 {
        value = *register_a;
    } else if operand == 5 {
        value = *register_b;
    } else if operand == 6 {
        value = *register_c;
    }
    
    let base: i32 = 2;
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

fn parse_registers(a_str: &str, b_str: &str, c_str: &str, register_a: &mut i32, register_b: &mut i32, register_c: &mut i32) {
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