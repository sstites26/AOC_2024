//const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let line = INPUT.lines().nth(0).expect("bad index 0");
    println!("{:?}", line);
    let numbers = process_line(line);

    let mut index = 0;
    let mut new_stones = change_stones(numbers.clone());

    while index < 74 {
        // println!("{:?}", new_stones);
        new_stones = change_stones(new_stones.clone());
        index += 1;
    }

    println!("{}", new_stones.len());
}

fn change_stones(numbers: Vec<i64>) -> Vec<i64> {
    let mut new_stones : Vec<i64> = Vec::new();

    for n in numbers {
        if n == 0 {
            new_stones.push(1);
        } else {
            let digit_count = get_digit_count(n);
            if digit_count % 2 == 0 {
                let split_nums = get_digits(n);
                new_stones.push(split_nums.0);
                new_stones.push(split_nums.1);
            } else {
                new_stones.push((n * 2024).try_into().unwrap());
            }
        }
    }

    new_stones
}

fn get_digits(number: i64) -> (i64, i64) {
    let num_string = number.to_string();
    let str_len = num_string.len();
    let first_half = &num_string[0..str_len / 2];
    let second_half = &num_string[str_len / 2..str_len];

    (first_half.parse::<i64>().unwrap(), second_half.parse::<i64>().unwrap())
}

fn get_digit_count(number: i64) -> i32 {
    let count = number.checked_ilog10().unwrap_or(0) + 1;
    count.try_into().unwrap()
}

fn process_line(line: &str) -> Vec<i64> {
    let numbers: Vec<i64> = line.split(' ').map(|x|->i64{x.parse().unwrap()}).collect();

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_digits_test() {
        let mut nums = get_digits(4567);
        assert_eq!(nums, (45, 67));

        nums = get_digits(67);
        assert_eq!(nums, (6, 7));

        nums = get_digits(123456);
        assert_eq!(nums, (123, 456));

        nums = get_digits(12345678);
        assert_eq!(nums, (1234, 5678));

        nums = get_digits(98765432);
        assert_eq!(nums, (9876, 5432));
    }

    #[test]
    fn get_digit_count_test() {
        let mut count = get_digit_count(45);
        assert_eq!(count, 2);
        
        count = get_digit_count(45);
        assert_eq!(count, 2);
        
        count = get_digit_count(4);
        assert_eq!(count, 1);
        
        count = get_digit_count(435);
        assert_eq!(count, 3);
        
        count = get_digit_count(4512);
        assert_eq!(count, 4);
        
        count = get_digit_count(12345);
        assert_eq!(count, 5);
    }
}