use cgmath::Point2;
use grid::Grid;
use pathfinding::prelude::dijkstra;

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt"); 

#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
    direction: Direction
}

impl Pos {
    fn successors(&self, grid: &Grid<char>) -> Vec<(Pos, usize)> {
        let &Pos{x, y, direction} = self;
        let mut weight = 1;

        let mut successors = Vec::new();
        
        let directions = [
            Pos{x:1, y:0, direction:Direction::Right},  
            Pos{x:-1, y:0, direction:Direction::Left}, 
            Pos{x:0, y:1, direction:Direction::Up},  
            Pos{x:0, y:-1, direction:Direction::Down}, 
        ];

        for (dx, dy, dir) in directions.iter() {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_y >= 0 && new_y < grid.rows() as i32 {
                if new_x >= 0 && new_x < grid.cols() as i32 {
                    if grid[(new_y as usize, new_x as usize)] != '#' {
                        successors.push(Pos{x:new_x, y:new_y, direction: Direction::Up});
                        weight = 2;
                    }
                }
            }
        }

        successors.into_iter().map(|p| (p, weight)).collect()
    }
}

fn main() {
    let rows =INPUT.lines().count();
    let cols = INPUT.lines().nth(0).expect("bad index 0").len();

    // let rows = 71;
    // let cols = 71;

    let mut grid : Grid<char> = Grid::new(rows, cols);
    grid.fill('.');

    let mut points : Vec<Point2<i32>> = Vec::new();
    let mut start_point = Point2{x: -1, y:-1};
    let mut end_point = Point2{x: -1, y:-1};

    let mut row_index = 0;
    for line in INPUT.lines() {

        if line.contains("E") {
            let end_index = line.find("E").unwrap();
            end_point = Point2{x: end_index as i32, y: row_index as i32};
        }

        if line.contains("S") {
            let start_index = line.find("S").unwrap();
            start_point = Point2{x: start_index as i32, y: row_index as i32};
        }

        add_line(line, &mut grid, row_index);
        row_index += 1;
    }

    print_grid(grid.clone());

    let result = dijkstra(&Pos{x:start_point.x, y:start_point.y, direction: Direction::Up}, |p| p.successors(&grid), |p| ((*p).x == end_point.x && (*p).y == end_point.y));
    println!("{:?}", result);
}

fn add_line(line: &str, grid: &mut Grid<char>, index: i32) {
    let mut numbers: Vec<char> = line.chars().map(|x|->char{x}).collect();
    grid.insert_row(index.try_into().unwrap(), numbers);
    grid.remove_row(grid.rows() - 1);
}

fn print_grid( grid: Grid<char>) {
    let mut index = 0;
    for row in grid.iter_rows() {
            for c in row {
                print!("{}", c);
            }
            println!("");
        index += 1;
    }
}