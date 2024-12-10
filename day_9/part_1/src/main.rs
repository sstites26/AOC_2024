const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt");

struct FileBlock {
    id: i32,
    file_count: i32,
    space_count: i32,
    forward: bool
}

fn main() {
    let line = INPUT.lines().nth(0).expect("bad index");
    println!("{}", line);

    let info_size = 2;
    let mut set_index = 0;

    let mut first_info = &line[0..info_size];
    let mut last_info = &line[line.len() - set_index - 1..];
    println!("{} {}", first_info, last_info);

    let set_pairs_left = (line.len() - 3) / 4;
    let mut sets_pairs_parsed = 0;


    
    // 23       2
    // 
    let mut first_info_index = set_index;
    let mut last_info_index = line.len() / 2;

    println!("first set index {} last set index {}", first_info_index, last_info_index);

    set_index += 1;
    first_info_index += 1;
    last_info_index -= 1;

    while sets_pairs_parsed < set_pairs_left {
        let pairs = get_next_pair(line, set_index.try_into().unwrap());
        println!("{} (index {}) {} (index {})", pairs.0, first_info_index, pairs.1, last_info_index);

        sets_pairs_parsed += 1;
        set_index += 1;
        first_info_index += 1;
        last_info_index -= 1;
    }
}

fn get_checksum(numbers: Vec<i32>, index_offset: i32) -> i64 {
    let mut index = 0;

    let mut checksum = 0;
    while index < numbers.len() {
        checksum += (numbers[index] * (((index as i32) * (index_offset as i32)) as i32)) as i64;
        index += 1;
    }

    return checksum;
}

fn get_next_pair(line: &str, index: i32) -> (i32, i32) { 
    let info_size = 2 as usize;
    let new_index = index as usize;
    let first_start = (new_index * info_size) as usize;
    let second_start = (line.len() - (new_index * 2) - 1) as usize;

    let first_info = &line[first_start..first_start + info_size];
    let last_info = &line[second_start..second_start + info_size];

    ((*first_info).parse::<i32>().unwrap(), (*last_info).parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_test() {
        let mut v = vec![1, 2, 3]; 
        let result = get_checksum(v, 1);
        assert_eq!(result, 8);
        
        v = vec![0,0,9,9,8,1,1,1,8,8,8,2,7,7,7,3,3,3,6,4,4,6,5,5,5,5,6,6]; 
        let result = get_checksum(v, 1);
        assert_eq!(result, 1928);

        v = vec![1, 2, 3]; 
        let result = get_checksum(v, 4);
        assert_eq!(result, 32);
    }
}