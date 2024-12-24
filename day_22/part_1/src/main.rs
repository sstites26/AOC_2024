// const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut numbers : Vec<i64> = Vec::new();

    for line in INPUT.lines() {
       numbers.push(line.parse::<i64>().unwrap());
    }

    let mut total = 0;
    for number in numbers {
        let mut index = 0;
        let mut n = number;

        while index < 2000 {
            n = process_number(n);
            index += 1; 
        } 

        total += n;
    }
    println!("{:?}", total);
}

fn process_number(number: i64) -> i64 {
    let step1_result = step_1(number);
    let step2_result = step_2(step1_result);
    let step3_result = step_3(step2_result);

    step3_result
}

fn step_3(secret: i64) -> i64 {
    let mut result = secret * 2048;
    result = mix_number(secret, result);
    result = prune_number(result);

    result
}

fn step_2(secret: i64) -> i64 {
    let mut result = secret / 32;
    result = mix_number(secret, result);
    result = prune_number(result);

    result
}

fn step_1(secret: i64) -> i64 {
    let mut result = secret * 64;
    result = mix_number(secret, result);
    result = prune_number(result);

    result
}

fn mix_number(secret: i64, value: i64) -> i64 {
    let result = secret ^ value;

    result
}

fn prune_number(secret: i64) -> i64 {
    let result = secret % 16777216;
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_number_test() {
        let result = mix_number(42, 15);
        assert_eq!(result, 37);
    }

    #[test]
    fn prune_number_test() {
        let result = prune_number(100000000);
        assert_eq!(result, 16113920);
    }
}