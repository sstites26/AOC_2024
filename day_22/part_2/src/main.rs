const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut numbers : Vec<i64> = Vec::new();

    for line in INPUT.lines() {
       numbers.push(line.parse::<i64>().unwrap());
    }

    let mut total = 0;
    for number in numbers {
        let mut max = 0;
        let mut max_index = 0;
        let mut prices : Vec<i32> = Vec::new();
        prices.push(get_digit(number));

        let mut index = 0;
        let mut n = number;

        while index < 9 {
            n = process_number(n);
            index += 1; 

            let digit = get_digit(n);
            prices.push(digit);

            if index > 3 {
                if digit > max {
                    max = digit;
                    max_index = index;
                }
            }
        } 

        println!("{:?} Max: {} Max Index: {}", prices, max, max_index);
    }
}

fn get_digit(number: i64) -> i32 {
    let num_string = number.to_string();
    let str_len = num_string.len();
    let last_digit = &num_string[str_len - 1..str_len];

    last_digit.parse::<i32>().unwrap()
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

    #[test]
    fn get_digit_test() {
        let mut result = get_digit(123);
        assert_eq!(result, 3);
        
        result = get_digit(4567894);
        assert_eq!(result, 4);

        result = get_digit(86845601234);
        assert_eq!(result, 4);
    }
}