use cgmath::Point2;
use grid::Grid;

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct MyPoint {
    point: Point2<i32>,
    value: i32
}

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();
    let mut grid : Grid<char> = Grid::new(line_count.try_into().unwrap(), line_width.try_into().unwrap());

    let mut start_point : Point2<i32> = Point2{x: 0, y: 0};
    let mut end_point : Point2<i32> = Point2{x: 0, y: 0};
    
    const RADIX: u32 = 10;

    let mut row_index = 0;
    for line in INPUT.lines() {
        let numbers: Vec<char> = line.chars().collect();
        add_line(&mut grid, line, numbers.clone(), row_index);
        let mut col_index = 0;
        for c in numbers {
            if c == 'S' {
                start_point = Point2{x: col_index, y: row_index};
            } else if c == 'E' {
                end_point = Point2{x: col_index, y: row_index};
            }
            col_index += 1;
        }
        row_index += 1;
    }

    let mut current_point = Point2{x: start_point.x, y: start_point.y};

    let mut neighbors : Vec<Point2<i32>> = Vec::new();
    let mut visited : Vec<Point2<i32>> = Vec::new();
    let mut path_count = 0;

    check_node(grid, current_point, path_count, &mut visited, &mut neighbors);
    
    // println!("Start: {:?} End: {:?}", start_point, end_point);

    // print_grid(&grid);
    // println!("Final {}", total);
}

fn check_node(grid: Grid<char>, current_point: Point2<i32>, path_count: i32, visited: &mut Vec<Point2<i32>>, neighbors: &mut Vec<Point2<i32>>) -> i32 {
    if grid[(current_point.y as usize, current_point.x as usize)] == 'E' {
        println!("FOUND E");
        return path_count;
    } 

    visited.push(Point2{x:current_point.x, y:current_point.y});

    // println!("Checking node: {:?}", current_point);
    get_neighbor(grid.clone(), current_point, path_count, visited, neighbors);

    // println!("Visited: {:?}", visited);

    while !neighbors.is_empty() {
        let mut node = neighbors.pop();

        if !visited.contains(&node.unwrap()) {
            check_node(grid.clone(), node.expect("REASON"), path_count, visited, neighbors);
        }
    }

    return 0;
}

fn get_neighbor(grid: Grid<char>, current_point: Point2<i32>, path_count: i32, visited: &mut Vec<Point2<i32>>, neighbors: &mut Vec<Point2<i32>>) {
    let start_point_x = current_point.x;
    let start_point_y = current_point.y;

    let mut check_point_x = start_point_x;
    let mut check_point_y = start_point_y - 1;
    let mut check_point = grid[(check_point_y as usize, check_point_x as usize)];

    let orig_neighbor_size = neighbors.len();

    // println!("AT {:?} Visited: {:?} Locally: {:?}", current_point, visited, locally_visited);

    if check_point != '#' && check_point != 'S' {
        neighbors.push(Point2{x: check_point_x, y: check_point_y});
    }

    check_point_x = start_point_x;
    check_point_y = start_point_y + 1;
    check_point = grid[(check_point_y as usize, check_point_x as usize)];
    if check_point != '#' && check_point != 'S' {
        neighbors.push(Point2{x: check_point_x, y: check_point_y});
    }

    check_point_x = start_point_x + 1;
    check_point_y = start_point_y;
    check_point = grid[(check_point_y as usize, check_point_x as usize)];
    if check_point != '#' && check_point != 'S' {
        neighbors.push(Point2{x: check_point_x, y: check_point_y});
    }

    check_point_x = start_point_x - 1;
    check_point_y = start_point_y;
    check_point = grid[(check_point_y as usize, check_point_x as usize)];
    if check_point != '#' && check_point != 'S' {
        neighbors.push(Point2{x: check_point_x, y: check_point_y});
    }
}

fn add_line(grid: &mut Grid<char>, line: &str, numbers: Vec<char>, index: i32) {
    grid.insert_row(index.try_into().unwrap(), numbers);
    grid.remove_row(grid.rows() - 1);
}

fn print_grid(grid: &Grid<char>) {
    let mut index = 0;
    for row in grid.iter_rows() {
            println!("{:?}", row);
        index += 1;
    }
}