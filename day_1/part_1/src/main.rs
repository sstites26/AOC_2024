const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut v1 : Vec<i32> = Vec::new();
    let mut v2 : Vec<i32> = Vec::new();

    INPUT.lines().for_each(|line| process_line(line, &mut v1, &mut v2));

    v1.sort();
    v2.sort();

    let mut index = 0;
    let mut total = 0;
    while index < v1.len() {
        let one = v1[index];
        let two = v2[index];

       // println!("{} {}", one, two);
        index = index + 1;

        let mut diff = two - one;
        if diff < 0 {
            diff = diff * -1;
        }
        total = total + diff;
    }

    println!("{}", total);
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