use geo::Point;
use grid::Grid;
use std::collections::HashMap;
use itertools::Itertools;

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();
    let mut grid : Grid<char> = Grid::new(line_count.try_into().unwrap(), line_width.try_into().unwrap());
    let mut char_map: HashMap<char, Vec<Point>> = HashMap::new();
    let mut y_map: HashMap<char, HashMap<i32, Vec<i32>>> = HashMap::new();
    let mut x_map: HashMap<char, HashMap<i32, Vec<i32>>> = HashMap::new();

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

    println!("\n");

    let mut total = 0;

    for (key, value) in char_map.clone().into_iter() {
        let mut points = value;
        let mut neighbors : Vec<Point> = Vec::new();
        let mut visited : Vec<Point> = Vec::new();
        let mut new_list : Vec<Point> = Vec::new();
        
        let mut collected_points : Vec<Point> = Vec::new();
        for point in points {
            let mut area = 0 as i32;
            let mut perimeter = 0 as i32;
            new_list.clear();
            y_map.clear();
            x_map.clear();

            if !visited.contains(&point) {
                check_node(&grid, key, point, &mut neighbors, &mut visited, &mut perimeter, &mut area, &mut new_list);
                create_y_map(&mut y_map, key, new_list.clone());
                create_x_map(&mut x_map, key, new_list.clone());

                let x_value = x_map.get(&key);
                let y_value = y_map.get(&key);

                let x_sides = get_sides(x_value.unwrap().clone());
                let y_sides = get_sides(y_value.unwrap().clone());

                total += ((x_sides + y_sides) * area);
            }
        }
    }
}

fn get_sides(x_map: HashMap<i32, Vec<i32>>) -> i32 {
    let mut prev_vec = None;
    let mut index = 0;
    let mut sides = 0;

    for key in x_map.keys().sorted() {
        if prev_vec == None {
            sides += calc_end_line(x_map[key].clone());
        } else {
            sides += compare_vectors(prev_vec.expect("REASON"), x_map[key].clone());
        }

        if index == x_map.len() - 1 {
            sides += calc_end_line(x_map[key].clone());
        }

        prev_vec = Some(x_map[key].clone());
        index += 1;
    }

    sides
}

fn calc_end_line(v1: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut prev_val = -1;
    let mut sides = 1;

    while index < v1.len() {
        let cur_val = v1[index];
        if prev_val != -1 && (prev_val - cur_val).abs() > 1 {
            sides += 1;
        }

        prev_val = cur_val;
        index += 1;
    }

    return sides;
}

fn compare_vectors(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut prev_val = -1;
    let mut sides = 0;let mut above = false;

    while index < v1.len() {
        let cur_val = v1[index];
        if prev_val == -1 && !v1.contains(&cur_val) {
            sides += 1;
        } else {
            if !v2.contains(&cur_val) && !above {
                above = true;
                sides += 1;
            } else {
                if prev_val != -1 && (prev_val - cur_val).abs() > 1 && !v2.contains(&cur_val) {
                    sides += 1;
                    above = true;
                } else if !v1.contains(&cur_val) && v1.contains(&prev_val) {
                    sides += 1;
                    above = true;
                } else if prev_val != -1 && (prev_val - cur_val).abs() > 1 || prev_val != -1 && v2.contains(&cur_val) {
                    above = false;
                }
            }
        }

        prev_val = cur_val;
        index += 1;
    }

    index = 0;
    prev_val = -1;
    above = false;

    while index < v2.len() {
        let cur_val = v2[index];

        if prev_val == -1 && !v2.contains(&cur_val) {
            sides += 1;
        } else {
            if !v1.contains(&cur_val) && !above {
                above = true;
                sides += 1;
            } else {
                if prev_val != -1 && (prev_val - cur_val).abs() > 1 && !v1.contains(&cur_val) {
                    sides += 1;
                    above = true;
                } else if !v2.contains(&cur_val) && v2.contains(&prev_val) {
                    sides += 1;
                    above = true;
                } else if prev_val != -1 && (prev_val - cur_val).abs() > 1|| prev_val != -1 && v1.contains(&cur_val)  {
                    above = false;
                }
            }
        }

        prev_val = cur_val;
        index += 1;
    }

    sides
}

fn create_x_map(x_map: &mut HashMap<char, HashMap<i32, Vec<i32>>>, key: char, points: Vec<Point>) {
    if !x_map.contains_key(&key) {
        let inner_hash : HashMap<i32, Vec<i32>> = HashMap::new();
        x_map.insert(key, inner_hash);
    }

    let hash = x_map.get_mut(&key).expect("REASON");
    
    for p in points {
        let x = p.x() as i32;

        if !hash.contains_key(&x) {
            let mut new_point_list : Vec<i32> = Vec::new();
            new_point_list.push(p.y() as i32);

            hash.insert(x, new_point_list);
        } else {
            let mut x_hash = hash.get_mut(&x).expect("REASON");
            x_hash.push(p.y() as i32);
            x_hash.sort();
        }
    }
}

fn create_y_map(y_map: &mut HashMap<char, HashMap<i32, Vec<i32>>>, key: char, points: Vec<Point>) {
    if !y_map.contains_key(&key) {
        let inner_hash : HashMap<i32, Vec<i32>> = HashMap::new();
        y_map.insert(key, inner_hash);
    }

    let hash = y_map.get_mut(&key).expect("REASON");
    
    for p in points {
        let y = p.y() as i32;

        if !hash.contains_key(&y) {
            let mut new_point_list : Vec<i32> = Vec::new();
            new_point_list.push(p.x() as i32);

            hash.insert(y, new_point_list);
        } else {
            let mut y_hash = hash.get_mut(&y).expect("REASON");
            y_hash.push(p.x() as i32);
            y_hash.sort();
        }
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

fn check_node(grid: &Grid<char>, char_key: char, p: Point, neighbors: &mut Vec<Point>, visited: &mut Vec<Point>, perimeter: &mut i32, area: &mut i32, new_list: &mut Vec<Point>) {
    let is_same_char = check_neighbor(grid, char_key, p, neighbors, visited, perimeter, area);
    visited.push(p);
    new_list.push(p);

    while !neighbors.is_empty() {
        let point = neighbors.pop();
        if !visited.contains(&point.unwrap()) {
            check_node(grid, char_key, point.expect("REASON"), neighbors, visited, perimeter, area, new_list);
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