use cgmath::Point2;
use grid::Grid;
use pathfinding::prelude::dijkstra;

// const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt"); 

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, grid: &Grid<char>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;

        let mut successors = Vec::new();
        
        let directions = [
            (1, 0),  
            (-1, 0), 
            (0, 1),  
            (0, -1), 
        ];

        for (dx, dy) in directions.iter() {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_y >= 0 && new_y < grid.rows() as i32 {
                if new_x >= 0 && new_x < grid.cols() as i32 {
                    if grid[(new_y as usize, new_x as usize)] != '#' {
                        successors.push(Pos(new_x, new_y));
                    }
                }
            }
        }

        successors.into_iter().map(|p| (p, 1)).collect()
    }
  }
  
  static GOAL: Pos = Pos(70, 70);
// static GOAL: Pos = Pos(6, 6);

fn main() {
    // let rows = 7;
    // let cols = 7;

    let rows = 71;
    let cols = 71;

    let mut grid : Grid<char> = Grid::new(rows, cols);
    grid.fill('.');

    let mut points : Vec<Point2<i32>> = Vec::new();

    for line in INPUT.lines() {
       points.push(process_line(line));
    }

    let mut index = 0;
    let end = INPUT.lines().count();

    while index < end {
        add_point(&mut grid, points[index]);
        
        let result = dijkstra(&Pos(0, 0), |p| p.successors(&grid), |p| *p == GOAL);
        
        if result == None {
            println!("{:?} {}", points[index], index);
            break;
        }
        index += 1;
    }
}

fn add_point(grid: &mut Grid<char>, point: Point2<i32>) {
    grid[(point.y as usize, point.x as usize)] = '#';
}

fn process_line(line: &str) -> Point2<i32> {
    let numbers: Vec<i32> = line.split(',').map(|x|->i32{x.parse().unwrap()}).collect();
    Point2{x: numbers[0], y: numbers[1]}
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