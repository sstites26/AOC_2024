const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut v1 : Vec<i32> = Vec::new();
    let mut v2 : Vec<i32> = Vec::new();

    INPUT.lines().for_each(|line| process_line(line, &mut v1, &mut v2));

    let mut count = 0;
    for value in v1 {
        count = count + result(value, &mut v2);

       // println!("{}", count);
    }

    println!("{}", count);
}

fn result(val1: i32, v2: &mut Vec<i32>) -> i32 {
    let count = v2.iter().filter(|&n| *n == val1).count();
    
    (count as i32) * val1
}

fn process_line(line: &str, v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    let parts = get_values(line);

    v1.push(parts.0);
    v2.push(parts.1);
}

fn get_values(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split("   ").collect();
    
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}