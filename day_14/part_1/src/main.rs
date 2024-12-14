use grid::Grid;
use cgmath::Point2;

// const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Robot {
    current_loc: Point2<i32>,
    vel_x: i32,
    vel_y: i32
}

fn main() {
    let rows = 7;
    let cols = 11;

    // let rows = 103;
    // let cols = 101;

    let mut grid : Grid<i32> = Grid::new(rows, cols);
    grid.fill(0);

    let mut robot_list : Vec<Robot> = Vec::new();

    INPUT.lines().for_each(|line| {
        robot_list.push(parse_robot(line));
    });

    let mut count = 0; 
    while count < 6351 {
        move_robots(&mut robot_list, rows.try_into().unwrap(), cols.try_into().unwrap());
        count += 1;
    }

    print_grid(grid, robot_list.clone());

    //count_robots(robot_list, rows.try_into().unwrap(), cols.try_into().unwrap());
}

fn count_robots(robot_list: Vec<Robot>, max_row: i32, max_col: i32) {
    let x_boundary = max_col / 2;
    let y_boundary = max_row / 2;

    let mut quad1 = 0;
    let mut quad2 = 0;
    let mut quad3 = 0;
    let mut quad4 = 0;

    for robot in robot_list {
        let current_x = robot.current_loc.x;
        let current_y = robot.current_loc.y;

        if current_x == x_boundary || current_y == y_boundary {
            // do not count;
        } else {
            if current_x < x_boundary && current_y < y_boundary {
                quad1 += 1;
            } else if current_x > x_boundary && current_y < y_boundary {
                quad2 += 1;
            } else if current_x < x_boundary && current_y > y_boundary {
                quad3 += 1;
            } else {
                quad4 += 1;
            }
        }
    }

    println!("1: {} 2: {} 3: {} 4: {}", quad1, quad2, quad3, quad4);
    let total = quad1 * quad2 * quad3 * quad4;
    println!("{}", total);
}

fn move_robots(robot_list: &mut Vec<Robot>, max_row: i32, max_col: i32) {
    for mut robot in robot_list {
        let current_x = robot.current_loc.x;
        let current_y = robot.current_loc.y;

        let mut next_x = current_x + robot.vel_x;
        let mut next_y = current_y + robot.vel_y;

        if next_x < 0 {
            let diff = next_x.abs();
            next_x = max_col - diff;
        } else if next_x >= max_col {
            let diff = next_x - max_col;
            next_x = diff;
        }

        if next_y < 0 {
            let diff = next_y.abs();
            next_y = max_row - diff;
        } else if next_y >= max_row {
            let diff = next_y - max_row;
            next_y = diff;
        }

        let new_point : Point2<i32> = Point2::new(next_x, next_y);
        robot.current_loc = new_point;
    }
}

fn parse_robot(line: &str) -> Robot {
    let parts: Vec<&str> = line.split(" ").collect();    
    
    let mut point = parts[0];
    point = &point[2..];
    
    let mut velocity = parts[1];
    velocity = &velocity[2..];

    let point_parts: Vec<i32> = point.split(',').map(|x|->i32{x.parse().unwrap()}).collect();
    let velocity_parts: Vec<i32> = velocity.split(',').map(|x|->i32{x.parse().unwrap()}).collect();

    let robot: Robot = Robot{ current_loc: Point2::new(point_parts[0], point_parts[1]), vel_x: velocity_parts[0], vel_y: 
        velocity_parts[1]};

    robot
}

fn printRobot(robot: Robot) {
    println!("Current: ({},{}) Veloctiy: ({},{})", robot.current_loc.x, robot.current_loc.y, robot.vel_x, robot.vel_y);
}

fn print_grid(mut grid: Grid<i32>, robot_list: Vec<Robot>) {
    grid.fill(0);

    for robot in robot_list {
        let x = robot.current_loc.x as usize;
        let y = robot.current_loc.y as usize;

        let robot_count: i32 = *grid.get(y, x).unwrap();
        grid[(y, x)] = robot_count + 1;
    }

    for row in grid.iter_rows() {
        for c in row {
            if *c == 0 {
                print!(".");
            } else {
                print!("&");
            }
        }
        println!("");
    }
}