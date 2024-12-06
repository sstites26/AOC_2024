use grid::Grid;
use std::collections::HashSet;

//const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt");

#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    direction: Direction
}

#[derive(Copy, Clone)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    NotVisited
}

#[derive(Copy, Clone)]
struct Guard {
    location: Point,
    direction: Direction
}

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();
    let mut grid : Grid<char> = Grid::new(line_count.try_into().unwrap(), line_width.try_into().unwrap());
    let mut points_visited : HashSet<Point> = HashSet::new();

    let mut guard = Guard { location: Point { x: 0, y: 0, direction: Direction::NotVisited}, direction: Direction::Up };
    let mut original_guard = Guard { location: Point { x: 0, y: 0, direction: Direction::NotVisited}, direction: Direction::Up };

    let mut row_index = 0;
    for line in INPUT.lines() {
        if line.contains('^') {
            guard.location = Point {x: line.chars().position(|r| r == '^').unwrap() as i32, y: row_index, direction: Direction::NotVisited };
            original_guard.location = Point {x: line.chars().position(|r| r == '^').unwrap() as i32, y: row_index, direction: Direction::NotVisited };
        }
        add_line(&mut grid, line, row_index);
        row_index += 1;
    }

    let rows = grid.rows();
    let columns = grid.cols();

    let mut row_index = 0;
    let mut loops = 0;
    while row_index < rows {
        let mut col_index = 0;
        while col_index < columns { 
            let current_char = grid[(row_index, col_index)];

            if current_char == '.' {
                let mut cloned_grid = grid.clone();
                points_visited.clear();

                cloned_grid[(row_index, col_index)] = '#';
                let is_it_loop = is_loop(guard, cloned_grid, &mut points_visited, line_count.try_into().unwrap(), line_width.try_into().unwrap());
                
                if is_it_loop {
                    loops += 1;
                }

                guard.location = Point { x: original_guard.location.x, y: original_guard.location.y, direction: original_guard.location.direction };
            }
             col_index += 1;
        }
        row_index += 1;
    }

    println!("{} {}", points_visited.len(), loops);
}

fn is_loop(mut guard: Guard, mut grid: Grid<char>, points_visited: &mut HashSet<Point>, line_count: i32, line_width: i32) -> bool {
    let mut moved = true;
    let mut done = false;

    while !done {
        moved = move_guard(&mut guard, &mut grid);

        if moved {
            let mut d;
            match guard.direction {
                Direction::Up => { d = Direction::Up }
                Direction::Down => { d = Direction::Down }
                Direction::Left => { d = Direction::Left }
                Direction::Right => { d = Direction::Right }
                Direction::NotVisited => { d = Direction::NotVisited }
            }

            let copy_point = Point { x: guard.location.x, y: guard.location.y, direction: d };

            if points_visited.contains(&copy_point) {
                return true;
            }
            points_visited.insert(copy_point);
        }
        done = check_boundaries(&guard, line_count, line_width);

        if done {
            return false;
        }
    }

    return false;
}

fn check_boundaries(guard: &Guard, line_count: i32, line_width: i32) -> bool {
    if guard.location.x == 0 || guard.location.x == line_width - 1 ||  guard.location.y == 0 || guard.location.y == line_count - 1 {
        return true;
    } else {
        return false;
    }
}

fn move_guard(guard: &mut Guard, grid: &mut Grid<char>) -> bool {
    if guard.direction == Direction::Up {
        return move_up(guard, grid)
    } else if guard.direction == Direction::Down {
        return move_down(guard, grid)
    } else if guard.direction == Direction::Left {
        return move_left(guard, grid)
    } else {
        return move_right(guard, grid)
    }
}

// returns true if the guard was able to move
fn move_up(guard: &mut Guard, grid: &mut Grid<char>) -> bool {
    let current_y = guard.location.y as i32;
    let current_x = guard.location.x as i32;

    if *(grid.get(current_y - 1, current_x).unwrap()) == '.' {
        guard.location = Point {x: current_x, y: current_y - 1, direction: Direction::Up };
        grid.swap(((current_y - 1).try_into().unwrap(), current_x.try_into().unwrap()), (current_y.try_into().unwrap(), current_x.try_into().unwrap()));
        
        return true;
    } else {
        guard.direction = Direction::Right;
        return false;
    }
}

fn move_down(guard: &mut Guard, grid: &mut Grid<char>) -> bool {
    let current_y = guard.location.y as i32;
    let current_x = guard.location.x as i32;

    if *(grid.get(current_y + 1, current_x).unwrap()) == '.' {
        guard.location = Point {x: current_x, y: current_y + 1, direction: Direction::Down };
        grid.swap(((current_y + 1).try_into().unwrap(), current_x.try_into().unwrap()), (current_y.try_into().unwrap(), current_x.try_into().unwrap()));
        return true;
    } else {
        guard.direction = Direction::Left;
        return false;
    }
}

fn move_left(guard: &mut Guard, grid: &mut Grid<char>) -> bool {
    let current_y = guard.location.y as i32;
    let current_x = guard.location.x as i32;

    if *(grid.get(current_y, current_x - 1).unwrap()) == '.' {
        guard.location = Point {x: current_x - 1, y: current_y, direction: Direction::Left };
        grid.swap((current_y.try_into().unwrap(), (current_x - 1).try_into().unwrap()), (current_y.try_into().unwrap(), current_x.try_into().unwrap()));
        return true;
    } else {
        guard.direction = Direction::Up;
        return false;
    }
}

fn move_right(guard: &mut Guard, grid: &mut Grid<char>) -> bool {
    let current_y = guard.location.y as i32;
    let current_x = guard.location.x as i32;

    if *(grid.get(current_y, current_x + 1).unwrap()) == '.' {
        guard.location = Point {x: current_x + 1, y: current_y, direction: Direction::Right };
        grid.swap((current_y.try_into().unwrap(), (current_x + 1).try_into().unwrap()), (current_y.try_into().unwrap(), current_x.try_into().unwrap()));
        return true;
    } else {
        guard.direction = Direction::Down;
        return false;
    }
}

fn add_line(grid: &mut Grid<char>, line: &str, index: i32) {
    let mut numbers: Vec<char> = line.chars().map(|x|->char{x}).collect();
    grid.insert_row(index.try_into().unwrap(), numbers);
    grid.remove_row(grid.rows() - 1);
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

fn print_guard(guard: &Guard) {
    println!("Guard is at ({},{}) and facing: {:?}", guard.location.x, guard.location.y, guard.direction);
}