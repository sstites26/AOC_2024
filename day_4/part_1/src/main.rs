const INPUT: &str = include_str!("test_input2.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let total_lines = INPUT.lines().count();

    let mut count = 0;
    let mut index = 0;

    while index < total_lines - 3 {
        let line1 = INPUT.lines().nth(index).expect("bad index 0");
        let line2 = INPUT.lines().nth(index + 1).expect("bad index 1");
        let line3 = INPUT.lines().nth(index + 2).expect("bad index 2");
        let line4 = INPUT.lines().nth(index + 3).expect("bad index 3");

        let dcount = get_diagnol_count(line1, line2, line3, line4);
        let vcount = get_vertical_count(line1, line2, line3, line4);

        count += dcount;
        count += vcount;

        index += 1;
    }

    index = 0;
    while index < total_lines {
        let hc = check_line(INPUT.lines().nth(index).expect("bad index"));

        count += hc;
        index += 1;
    }
    
    println!("{}", count);
}

fn get_vertical_count(line1: &str, line2: &str, line3: &str, line4: &str) -> i32 {
    let vertical_strings = get_vertical_strings(line1, line2, line3, line4);

    let mut count = 0;
    for d_string in vertical_strings {
        count += check_line(&d_string);
    }

    count
}

fn get_diagnol_count(line1: &str, line2: &str, line3: &str, line4: &str) -> i32 {
    let diagnol_strings = get_diagnol_strings(line1, line2, line3, line4);

    let mut count = 0;
    for d_string in diagnol_strings {
        count += check_line(&d_string);
    }
    
    let reversed_diagnol_strings = get_diagnol_strings(&reverse(line1), &reverse(line2), &reverse(line3), &reverse(line4));
    for d_string in reversed_diagnol_strings {
        count += check_line(&d_string);
    }

    count
}

fn get_diagnol_strings(line1: &str, line2: &str, line3: &str, line4: &str) -> Vec<String> {
    let mut index = 0;
    let mut diagnol_strings : Vec<String> = Vec::new();

    while index < line1.len() - 3 {
        let mut temp_string = String::from("");
        temp_string.push(line1.chars().collect::<Vec<_>>()[index]);
        temp_string.push(line2.chars().collect::<Vec<_>>()[index + 1]);
        temp_string.push(line3.chars().collect::<Vec<_>>()[index + 2]);
        temp_string.push(line4.chars().collect::<Vec<_>>()[index + 3]);

        index += 1;
        diagnol_strings.push(temp_string);
    }

    diagnol_strings
}

fn get_vertical_strings(line1: &str, line2: &str, line3: &str, line4: &str) -> Vec<String> {
    let mut index = 0;
    let mut vertical_strings : Vec<String> = Vec::new();

    while index < line1.len() {
        let mut temp_string = String::from("");
        temp_string.push(line1.chars().collect::<Vec<_>>()[index]);
        temp_string.push(line2.chars().collect::<Vec<_>>()[index]);
        temp_string.push(line3.chars().collect::<Vec<_>>()[index]);
        temp_string.push(line4.chars().collect::<Vec<_>>()[index]);

        index += 1;
        vertical_strings.push(temp_string);
    }

    vertical_strings
}

fn check_line(line: &str) -> i32 {
    let forward = xmas_word_count(line);
    let backward = check_backwards(line);

    //println!("{} - f:{} b:{}", line, forward, backward);

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
    let c = line.matches("XMAS").count();
    c as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmas_word_count_test() {
        let mut count = xmas_word_count("XMASXMASNVMSXMAS");
        assert_eq!(count, 3);

        count = xmas_word_count("   JFMASXXMAS");
        assert_eq!(count, 1);
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
    fn get_vertical_count_test() {
        let mut count = get_vertical_count("..A.A.MS.X", "XMASAMX.MM", "X.....XA.A", "S.S.S.S.SS");
        assert_eq!(count, 1);
    }
}