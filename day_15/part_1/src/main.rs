use grid::Grid;
use cgmath::Point2;

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Moveable {
    open_point: Point2<i32>,
    points_to_move: Vec<Point2<i32>>
}

fn main() {
    let mut is_grid_info = true;
    
    let mut grid_lines : Vec<&str> = Vec::new();
    let mut directions : Vec<&str> = Vec::new();

    INPUT.lines().for_each(|line| {
        if line == "" {
            is_grid_info = false;
        } else {
            if is_grid_info {
                grid_lines.push(line);
            } else {
                directions.push(line);
            }
        }
    });

    let instructions = parse_directions(directions.clone());
    let parsed = parse_grid(grid_lines.clone());
    let mut grid = parsed.0;
    let mut robot = parsed.1;
    
    let mut instruction_index = 0;

    while instruction_index < instructions.len() {
        let instruction = instructions[instruction_index];
        let mut direction = Direction::Right;

        if instruction == '^' {
            direction = Direction::Up;
        } else if instruction == 'v' {
            direction = Direction::Down;
        } else if instruction == '<' {
            direction = Direction::Left;
        } 
        
        let new_point = process_move(&mut grid, robot, direction);
        robot = new_point;
        instruction_index += 1;
    }

    let mut row = 0;
    let mut col = 0;

    let mut total = 0;
    while row < grid.rows() - 1 {
        row += 1;
        col = 0;
        while col < grid.cols() - 1 {
            if grid[(row, col)] == 'O' {
                total += ((row * 100) + col);
            }
            col += 1;
        }
    }

    println!("Total: {}", total);
}

fn process_move(grid: &mut Grid<char>, mut robot: Point2<i32>, direction: Direction) -> Point2<i32> {
    let can_move = can_move(grid.clone(), robot, direction.clone());
    let the_points = can_move.0.points_to_move;
    let mut index = 0;
    
    if the_points.len() > 0 {
        index = the_points.len() - 1;
    }

    if the_points.len() == 0 && can_move.1 {
        move_point(grid, &mut robot, direction.clone(), true);
        return robot;
    }
    
    while index >= 0 {
        let mut point = the_points[index];
        let mut is_robot = true;

        if grid[(point.y as usize, point.x as usize)] == 'O' {
            is_robot = false;
        }

        let new_point = move_point(grid, &mut point, direction.clone(), is_robot);

        if is_robot {
            robot = new_point;
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    return robot;
}

fn can_move(grid: Grid<char>, starting_point: Point2<i32>, direction: Direction) -> (Moveable, bool) {
    let mut done = false;
    let mut count = 0;
    let mut can_move = false;

    let mut current_x = starting_point.x;
    let mut current_y = starting_point.y;

    let mut diff_x = 0;
    let mut diff_y = 0;
    
    let mut points_to_move : Vec<Point2<i32>> = Vec::new();

    if direction == Direction::Up {
        diff_y = -1;
    } else if direction == Direction::Down {
        diff_y = 1;
    } else if direction == Direction::Left {
        diff_x = -1;
    } else if direction == Direction::Right {
        diff_x = 1;
    }

    while !done {
        if current_x >= grid.cols().try_into().unwrap() || current_y >= grid.rows().try_into().unwrap() ||  grid[(current_y as usize, current_x as usize)] == '#' {
            return (Moveable{open_point: Point2{x: starting_point.x, y: starting_point.y}, points_to_move: points_to_move}, can_move);
        } else if grid[(current_y as usize, current_x as usize)] == '.' {
            can_move = true;
            return (Moveable{open_point: Point2{x: current_x, y: current_y}, points_to_move: points_to_move}, can_move);
        }

        let new_point = Point2{x:current_x, y: current_y};
        points_to_move.push(new_point);

        count += 1;
        current_x += diff_x;
        current_y += diff_y;
    }
    
    (Moveable{open_point: Point2{x: starting_point.x, y: starting_point.y}, points_to_move: points_to_move}, can_move)
}

fn move_point(grid: &mut Grid<char>, point: &mut Point2<i32>, direction: Direction, is_robot: bool) -> Point2<i32> {
    let orig_x = point.x;
    let orig_y = point.y;

    let mut symbol = '@';
    if !is_robot {
        symbol = 'O';
    }

    let mut diff_x = 0;
    let mut diff_y = 0;

    if direction == Direction::Up {
        diff_y = -1;
    } else if direction == Direction::Down {
        diff_y = 1;
    } else if direction == Direction::Left {
        diff_x = -1;
    } else if direction == Direction::Right {
        diff_x = 1;
    }

    let above_x = orig_x + diff_x;
    let above_y = orig_y + diff_y;

    if *grid.get(above_y, above_x).unwrap() == '.' {
        *point = Point2{x: above_x, y: above_y};
        grid[(above_y as usize, above_x as usize)] = symbol;
        grid[(orig_y as usize, orig_x as usize)] = '.';

        return Point2{x: above_x, y: above_y};
    }

    return *point;
}

fn parse_grid(grid_lines: Vec<&str>) -> (Grid<char>, Point2<i32>) {
    let mut grid : Grid<char> = Grid::new(grid_lines.len().try_into().unwrap(), grid_lines[0].len().try_into().unwrap());
    let mut robot_pos : Point2<i32> = Point2{x: 0, y: 0};

    let mut row = 0;
    for line in grid_lines {
        let pos = line.chars().position(|c| c == '@');

        if pos != None {
            robot_pos = Point2{x: pos.unwrap() as i32, y: row.try_into().unwrap()};
        }

        grid.insert_row(row.try_into().unwrap(), line.chars().collect());
        grid.remove_row(grid.rows() - 1);
        row += 1;
    }

    (grid, robot_pos)
}

fn parse_directions(dir_string: Vec<&str>) -> Vec<char> {
    let mut direction_chars : Vec<char> = Vec::new();

    for string in dir_string {
        let mut char_vec: Vec<char> = string.chars().collect();
        direction_chars.append(&mut char_vec);
    }

    direction_chars
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