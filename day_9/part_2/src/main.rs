
use std::fs::File;
use std::io::{BufWriter, Write};

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct MyFile {
    index: i64,
    block_size: i64,
    space_after: i64,
    display: i64
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Available_Info {
    starting_index: i64,
    sizes: i64
}

fn main() {
    let line = INPUT.lines().nth(0).expect("bad index");
    let vector_values = parse_line(line);

    let vv = sort_it(vector_values.1, vector_values.0);

    let mut total = 0;
    let mut value_index = 0;
    for value in vv {
        if value != -1 {
            let temp = value * value_index;
            total += temp;
        }
        value_index += 1;
    }

     println!("{}", total);
}

fn sort_it(mut files_info: Vec<MyFile>, mut values: Vec<i64>) -> Vec<i64> {
    let mut file = files_info.pop();
    let mut test = 0;
    while file != None {
        let mut moved = false;
        let mut jklm = 0;
        while !moved {
            let available = how_many_spaces(values.clone(), jklm);

            if jklm > file.unwrap().index || file.unwrap().index < available.starting_index {
                moved = true;
            } else if available.sizes >= file.unwrap().block_size  {
                values = move_block(values, available.starting_index, file.unwrap().index, file.unwrap().block_size);
                moved = true;
                jklm = 0;
            } else  {
                jklm = available.starting_index + available.sizes + 1;
            }
        }
        
        file = files_info.pop();

        test += 1;
    }
    values
}

fn move_block(mut line: Vec<i64>, into_index: i64, from_index: i64, size: i64) -> Vec<i64>{
    let mut swaps = 0;
    while swaps < size {
        line.swap((into_index + swaps).try_into().unwrap(), (from_index + swaps).try_into().unwrap());
        swaps += 1;
    }

    line
}

fn how_many_spaces(line: Vec<i64>, index: i64) -> Available_Info {
    let mut i = index;
    let mut done = false;
    let mut replace_index = index;
    let mut count = 1;

    while !done {
        if *line.get(i as usize).unwrap() == -1 {
            replace_index = i;
            let mut new_i = i as usize;
            while *line.get(new_i).unwrap() == -1 && new_i < line.len() - 1 {
                count += 1;
                new_i += 1;
            }
            done = true;
        }
        i += 1;
    }

    count = count - 1;
    let available_info = Available_Info{starting_index: replace_index, sizes: count};

    available_info
}

fn parse_line(line: &str) -> (Vec<i64>, Vec<MyFile>) {
    let mut index = 0;
    let last_string = line.chars().last().unwrap();
    let mut vector_vals : Vec<i64> = Vec::new();
    let mut files : Vec<MyFile> = Vec::new();
    let mut index_value = 0;

    while index < line.len() - 2 {
        let pair = &line[index..index + 2];
        let digits = get_digits(pair.parse::<i64>().expect("REASON"));

        let mut i = 0;

        let file = MyFile{index: vector_vals.len() as i64, block_size: digits.0, space_after: digits.1, display: index_value};
        // println!("{:?}", file);
        files.push(file);

        while i < digits.0 {
            vector_vals.push(index_value);
            i += 1;
        }

        i = 0;
        while i < digits.1 {
            vector_vals.push(-1);
            i += 1;
        }

        index += 2;
        index_value += 1;
    }

    let mut i = 0;
    let last_value = (last_string.to_string()).parse::<i64>().unwrap();

    let file = MyFile{index: vector_vals.len() as i64, block_size: last_value, space_after: 0, display: index_value};
    files.push(file);

    while i < last_value {
         vector_vals.push(index_value);
         i += 1;
    }

    (vector_vals, files)
}

fn get_digits(number: i64) -> (i64, i64) {
    let first = number / 10;
    let second = number % 10;

    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_test() {
        
        let mut first = [2 , 2, 2, -1, -1, -1, 5, 7, 7, 7];
        let mut result = move_block(first.to_vec(), 3, 7, 3);
        assert_eq!(result, [2 , 2, 2, 7, 7, 7, 5, -1, -1, -1]);

        let mut second = [2 , 2, 2, 4, 1, -1, 5, 7, 7, 7, 6, 6, 6, -1, -1, 8, 8, 9, 9, 9];
        result = move_block(second.to_vec(), 13, 15, 2);
        assert_eq!(result, [2 , 2, 2, 4, 1, -1, 5, 7, 7, 7, 6, 6, 6, 8, 8, -1, -1, 9, 9, 9]);

        let mut third = [-1 , 2, 2, 4, 1, -1, 5];
        result = move_block(third.to_vec(), 0, 6, 1);
        assert_eq!(result, [5, 2 , 2, 4, 1, -1, -1]);
    }
}