use grid::Grid;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
struct KeyLock {
    grid: Grid<char>,
    pins: Vec<i32>
}

// const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let count = INPUT.lines().count();
    let line_width = INPUT.lines().nth(0).expect("DOESNT EXIST").len();

    let mut line_index = 0;

    let mut keys : Vec<KeyLock> = Vec::new();
    let mut locks : Vec<KeyLock> = Vec::new();

    while line_index < count {
        
        let mut grid : Grid<char> = Grid::new(5, line_width.try_into().unwrap());
        let l1 = INPUT.lines().nth(line_index).expect("REASON");
        let l7 = INPUT.lines().nth(line_index + 6).expect("REASON");

        let mut row_index = 1; 

        while row_index < 7 {
            let row = INPUT.lines().nth(row_index + line_index).expect("REASON");
            add_line(&mut grid, row, (row_index - 1).try_into().unwrap());
            row_index += 1;
        }

        if l1 == "....." {
            process_key(grid, &mut keys);
        } else {
            process_lock(grid, &mut locks);
        }

        line_index += 8;
    }

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if check_lock(lock.pins.clone(), key.pins.clone()) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn check_lock(lock: Vec<i32>, key: Vec<i32>) -> bool {
    let mut index = 0;

    while index < 5 {
        let total = lock[index] + key[index];
        if total > 5 {
            return false;
        }

        index += 1;
    }

    return true;
}

fn add_line(grid: &mut Grid<char>, line: &str, index: i32) {
    grid.insert_row(index.try_into().unwrap(), line.chars().collect());
    grid.remove_row(grid.rows() - 1);
}

fn process_key(grid: Grid<char>, keys: &mut Vec<KeyLock>) {
    let mut pins : Vec<i32> = Vec::new();

    let mut pin_col_index = 0;

    while pin_col_index < 5 {
        let mut pin_row_index = 4;
        let mut done = false;
        let mut pin_count = 0;

        while pin_row_index >= 0 && !done {
            let c = grid[(pin_row_index, pin_col_index)];

            if c == '.' {
                pins.push(pin_count);
                done = true;
            } else {
                pin_count += 1;

                if pin_row_index == 0 {
                    pins.push(pin_count);
                    break;
                }
            }
            if pin_row_index == 0 {
                break;
            } else {
                pin_row_index -= 1;
            }
        }
        pin_col_index += 1;
    }

    let key = KeyLock{grid: grid, pins: pins};
    keys.push(key);
}

fn process_lock(grid: Grid<char>, locks: &mut Vec<KeyLock>) {
    let mut pins : Vec<i32> = Vec::new();

    let mut pin_col_index = 0;

    while pin_col_index < 5 {
        let mut pin_row_index = 0;
        let mut done = false;
        let mut pin_count = 0;

        while pin_row_index < 5 && !done {
            let c = grid[(pin_row_index, pin_col_index)];

            if c == '.' {
                pins.push(pin_count);
                done = true;
            } else {
                pin_count += 1;

                if pin_row_index == 4 {
                    pins.push(pin_count);
                }
            }
            pin_row_index += 1;
        }
        pin_col_index += 1;
    }

    let lock = KeyLock{grid: grid, pins: pins};
    locks.push(lock);
}

fn print_grid(grid: &Grid<char>) {
    let rows = grid.rows();
    let columns = grid.cols();

    let mut row_index = 0;
    while row_index < rows {
        println!("{:?}", grid.iter_row(row_index)); 
        row_index += 1;
    }
}

fn print_key_lock(key: bool, key_lock: KeyLock) {
    if key {
        println!("KEY");
    } else {
        println!("LOCK");
    }

    println!("{:?}", key_lock.pins);
    print_grid(&key_lock.grid);
}