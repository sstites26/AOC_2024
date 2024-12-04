const INPUT: &str = include_str!("test_input2.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let total_lines = INPUT.lines().count();

    let mut count = 0;
    let mut line_index = 1;

    while line_index < total_lines - 1 {
        let main_line = INPUT.lines().nth(line_index).expect("bad index 1");
        let line1 = INPUT.lines().nth(line_index - 1).expect("bad index 0");
        let line3 = INPUT.lines().nth(line_index + 1).expect("bad index 2");

        let mut char_index = 0;

        for c in main_line.chars() {
            if c == 'A' && char_index > 0 && char_index < main_line.len() - 1 {
                if get_diagnol_count(line1, main_line, line3, char_index) {
                    count += 1;
                } 
            }
            char_index += 1;
        }

        line_index += 1;
    }
    
    println!("{}", count);
}

fn get_diagnol_count(line1: &str, line2: &str, line3: &str, center_index: usize) -> bool {
    let diagnol_strings = get_diagnol_strings(line1, line2, line3, center_index);

    let mut count = 0;
    for d_string in diagnol_strings {
        count += check_line(&d_string);
    }

    count == 2
}

fn get_diagnol_strings(line1: &str, line2: &str, line3: &str, center_index: usize) -> Vec<String> {
    let mut diagnol_strings : Vec<String> = Vec::new();

    let mut temp_string = String::from("");

    temp_string.push(line1.chars().collect::<Vec<_>>()[center_index - 1]);
    temp_string.push(line2.chars().collect::<Vec<_>>()[center_index]);
    temp_string.push(line3.chars().collect::<Vec<_>>()[center_index + 1]);
    
    diagnol_strings.push(temp_string);

    temp_string = String::from("");
    temp_string.push(line1.chars().collect::<Vec<_>>()[center_index + 1]);
    temp_string.push(line2.chars().collect::<Vec<_>>()[center_index]);
    temp_string.push(line3.chars().collect::<Vec<_>>()[center_index - 1]);
    
    diagnol_strings.push(temp_string);

    diagnol_strings
}

fn check_line(line: &str) -> i32 {
    let forward = xmas_word_count(line);
    let backward = check_backwards(line);

    forward + backward
}

fn check_backwards(line: &str) -> i32 {
    let reversed = reverse(line);
    xmas_word_count(&reversed) as i32
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn xmas_word_count(line: &str) -> i32 {
    let c = line.matches("MAS").count();
    c as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmas_word_count_test() {
        let mut count = xmas_word_count("XMASXMASNVMSAMXMAS");
        assert_eq!(count, 3);

        count = xmas_word_count("   JFMASXXMAS");
        assert_eq!(count, 2);
    }
    
    #[test]
    fn check_backwards_test() {
        let mut count = check_backwards("SAMXSAMXLKJSDF");
        assert_eq!(count, 2);

        count = check_backwards("   SMASX");
        assert_eq!(count, 0);
    }
    
    #[test]
    fn check_line_test() {
        let mut count = check_line("XMASAMX");
        assert_eq!(count, 2);

        count = check_line("SAMXMASAMXXSAMXSDFD");
        assert_eq!(count, 4);
    }
    
    #[test]
    fn get_diagnol_count_test() {
        let good = get_diagnol_count("M.S", ".A.", "M.S", 1);
        assert_eq!(good, true);
    }
}