use geo::Point;
use grid::Grid;
use std::collections::HashMap;

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();
    let mut grid : Grid<char> = Grid::new(line_count.try_into().unwrap(), line_width.try_into().unwrap());
    let mut char_map: HashMap<char, Vec<Point>> = HashMap::new();

    let mut row_index = 0;
    for line in INPUT.lines() {
        add_line(&mut grid, line, row_index);
        let mut col_index = 0;
        for c in line.chars() {
            let p = Point::new(col_index as f64, row_index as f64);
            if char_map.contains_key(&c) {
                char_map.get_mut(&c).expect("DOESNT EXIST").push(p);
            } else {
                let mut v : Vec<Point> = Vec::new();
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
        let mut neighbors : Vec<Point> = Vec::new();
        let mut visited : Vec<Point> = Vec::new();
        // println!("{}", key);
        
        for point in points {
            let mut area = 0 as i32;
            let mut perimeter = 0 as i32;
            if !visited.contains(&point) {
                check_node(&grid, key, point, &mut neighbors, &mut visited, &mut perimeter, &mut area);
                // println!("Area: {} Perimeter: {}\n", area, perimeter);
                total += area * perimeter; 
            }
        }
    }

    //print_grid(&grid);
    println!("Final {}", total);
}

fn check_node(grid: &Grid<char>, char_key: char, p: Point, neighbors: &mut Vec<Point>, visited: &mut Vec<Point>, perimeter: &mut i32, area: &mut i32) {
    // println!("{:?}", p);
    let is_same_char = check_neighbor(grid, char_key, p, neighbors, visited, perimeter, area);
    visited.push(p);

    while !neighbors.is_empty() {
        let point = neighbors.pop();
        if !visited.contains(&point.unwrap()) {
            check_node(grid, char_key, point.expect("REASON"), neighbors, visited, perimeter, area);
        }
    }
}

fn check_neighbor(grid: &Grid<char>, char_key: char, p: Point, neighbors: &mut Vec<Point>, visited: &mut Vec<Point>, perimeter: &mut i32, area: &mut i32) {
    let current_x = p.x() as i32;
    let current_y = p.y() as i32;
    *area += 1;

    let mut new_point = grid.get(current_y - 1, current_x);
    if !new_point.is_none() {
        if *new_point.unwrap() == char_key {
            let np = Point::new(current_x as f64, (current_y - 1) as f64);
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    // println!("Add above");
                    neighbors.push(np);
                }
            }
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }
    
    new_point = grid.get(current_y + 1, current_x);
    if !new_point.is_none() {
        if *new_point.unwrap() == char_key {
            let np = Point::new(current_x as f64, (current_y + 1) as f64);
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    // println!("Add below");
                    neighbors.push(np);
                }
            }
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }
    
    new_point = grid.get(current_y, current_x - 1);
    if !new_point.is_none() {
        if *new_point.unwrap() == char_key {
            let np = Point::new((current_x - 1) as f64, current_y as f64);
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    // println!("Add left");
                    neighbors.push(np);
                }
            }
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }
    
    new_point = grid.get(current_y, current_x + 1);
    if !new_point.is_none() {
        if *new_point.unwrap() == char_key {
            let np = Point::new((current_x + 1) as f64, current_y as f64);
            if !visited.contains(&np) {
                if !neighbors.contains(&np) {
                    // println!("Add right");
                    neighbors.push(np);
                }
            }
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
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
