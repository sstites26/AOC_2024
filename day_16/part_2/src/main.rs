use cgmath::Point2;
use grid::Grid;
use pathfinding::prelude::astar_bag;
use std::collections::HashSet;

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

        let mut successors = Vec::new();

        let directions = [
            Pos{x: 1, y: 0, direction: Direction::Right},  
            Pos{x: -1, y: 0, direction: Direction::Left}, 
            Pos{x: 0, y: 1, direction: Direction::Down},  
            Pos{x: 0, y: -1, direction: Direction::Up}, 
        ];

        for pos in directions.iter() {
            let new_x = x + pos.x;
            let new_y = y + pos.y;
            let new_dir = pos.direction;

            if new_y >= 0 && new_y < grid.rows() as i32 {
                if new_x >= 0 && new_x < grid.cols() as i32 {
                    if grid[(new_y as usize, new_x as usize)] != '#' {
                        let mut weight = 0;
                        if new_dir != self.direction { 
                            weight = 1001;
                        } else { 
                            weight = 1;
                        }
                        successors.push((Pos{x: new_x, y: new_y, direction: new_dir}, weight));
                    }
                }
            }
        }

        successors
    }
}

fn main() {
    let rows =INPUT.lines().count();
    let cols = INPUT.lines().nth(0).expect("bad index 0").len();

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

    // print_grid(grid.clone());

    let result = astar_bag(&Pos{x:start_point.x, y:start_point.y, direction: Direction::Right}, |p| p.successors(&grid), |p| (p.x * 1).try_into().unwrap(), |p| ((*p).x == end_point.x && (*p).y == end_point.y));
   
    let solutions = result.expect("no path found").0;

    let mut points_visited : HashSet<Point2<i32>> = HashSet::new();
    let mut index = 0;

    for s in solutions {
        for point in s {
            let new_p = Point2{x:point.x, y: point.y};
            points_visited.insert(new_p);
            index += 1;
        }
    }

    println!("all points {}", index);
    println!("new points {}", points_visited.len());
}

fn add_line(line: &str, grid: &mut Grid<char>, index: i32) {
    let numbers: Vec<char> = line.chars().map(|x|->char{x}).collect();
    grid.insert_row(index.try_into().unwrap(), numbers);
    grid.remove_row(grid.rows() - 1);
}

fn print_grid( grid: Grid<char>) {
    for row in grid.iter_rows() {
            for c in row {
                print!("{}", c);
            }
            println!("");
    }
}