use cgmath::Point2;
use grid::Grid;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct MyPoint {
    point: Point2<i32>,
    value: i32
}

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();
    let mut grid : Grid<i32> = Grid::new(line_count.try_into().unwrap(), line_width.try_into().unwrap());
    let mut char_map: HashMap<i32, Vec<MyPoint>> = HashMap::new();
    
    const RADIX: u32 = 10;

    let mut row_index = 0;
    for line in INPUT.lines() {
        let numbers: Vec<i32> = line.chars().map(|x|->i32{x.to_digit(RADIX).unwrap() as i32}).collect();
        add_line(&mut grid, line, numbers.clone(), row_index);
        let mut col_index = 0;
        for c in numbers {
            let pp: Point2<i32> = Point2::new(col_index, row_index);
            let p = MyPoint{point: pp, value: c};
            if char_map.contains_key(&c) {
                char_map.get_mut(&c).expect("DOESNT EXIST").push(p);
            } else {
                let mut v : Vec<MyPoint> = Vec::new();
                v.push(p);
                char_map.insert(c, v);
            }
            col_index += 1;
        }
        row_index += 1;
    }

    let mut total = 0;
    for (key, value) in char_map.into_iter() {
        let mut points = value;
        if key == 0 {
            for point in points {
                let mut neighbors : Vec<MyPoint> = Vec::new();
                let mut visited : Vec<MyPoint> = Vec::new();
                if !visited.contains(&point) {
                    let mut count = 0;
                    check_node(&grid, key, point, &mut neighbors, &mut visited, &mut count);
                    total += count;
                }
            }
        }
    }

    print_grid(&grid);
    println!("Final {}", total);
}

fn check_node(grid: &Grid<i32>, char_key: i32, p: MyPoint, neighbors: &mut Vec<MyPoint>, visited: &mut Vec<MyPoint>, count: &mut i32) {
    let is_same_char = check_neighbor(grid, p, neighbors, visited);
    visited.push(p);

    if p.value == 9 {
        *count = *count + 1;
    }

    while !neighbors.is_empty() {
        let point = neighbors.pop();
        if !visited.contains(&point.unwrap()) {
            check_node(grid, char_key, point.expect("REASON"), neighbors, visited, count);
        }
    }
}

fn check_neighbor(grid: &Grid<i32>, p: MyPoint, neighbors: &mut Vec<MyPoint>, visited: &mut Vec<MyPoint>) {
    let current_x = p.point.x;
    let current_y = p.point.y;

    let current_value = p.value;
    let looking_for = current_value + 1;

    let mut new_point = grid.get(current_y - 1, current_x);
    if !new_point.is_none() {
        if *new_point.unwrap() == looking_for {
            let pp: Point2<i32> = Point2::new(current_x, current_y - 1);
            let np = MyPoint{point: pp, value: *new_point.unwrap()};
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    neighbors.push(np);
                }
            }
        } 
    }
    
    new_point = grid.get(current_y + 1, current_x);
    if !new_point.is_none() {
        if *new_point.unwrap() == looking_for {
            let pp: Point2<i32> = Point2::new(current_x, current_y + 1);
            let np = MyPoint{point: pp, value: *new_point.unwrap()};
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    neighbors.push(np);
                }
            }
        }
    }
    
    new_point = grid.get(current_y, current_x - 1);
    if !new_point.is_none() {
        if *new_point.unwrap() == looking_for {
            let pp: Point2<i32> = Point2::new(current_x - 1, current_y);
            let np: MyPoint = MyPoint{point: pp, value: *new_point.unwrap()};
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    neighbors.push(np);
                }
            }
        }
    }
    
    new_point = grid.get(current_y, current_x + 1);
    if !new_point.is_none() {
        if *new_point.unwrap() == looking_for {
            let pp:Point2<i32> = Point2::new(current_x + 1, current_y);
            let np: MyPoint = MyPoint{point: pp, value: *new_point.unwrap()};
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    neighbors.push(np);
                }
            }
        }
    }
}

fn add_line(grid: &mut Grid<i32>, line: &str, numbers: Vec<i32>, index: i32) {
    grid.insert_row(index.try_into().unwrap(), numbers);
    grid.remove_row(grid.rows() - 1);
}

fn print_grid(grid: &Grid<i32>) {
    let rows = grid.rows();
    let columns = grid.cols();

    let mut row_index = 0;
    while row_index < rows {
        row_index += 1;
    }
}