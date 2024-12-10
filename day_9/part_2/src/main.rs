use grid::Grid;
use geo::Point;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt");

#[derive(Eq, Hash, PartialEq)]
struct MyPoint {
    x: i32,
    y: i32,
}

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();
    let mut grid : Grid<char> = Grid::new(line_count.try_into().unwrap(), line_width.try_into().unwrap());

    let mut point_map: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antennas : HashSet<MyPoint> = HashSet::new();
    
    let mut y = 0;
    INPUT.lines().for_each(|line| {
        let mut x = 0;

        for c in line.chars() {
            if c != '.' && c != '#' {
                let point = Point::new(x as f64, y as f64);

                if point_map.contains_key(&c) {
                    point_map.get_mut(&c).expect("REASON").push(point);
                } else {
                    let mut vect : Vec<Point> = Vec::new();
                    vect.push(point);
                    point_map.insert(c, vect);
                }
            }

            x += 1;
        }

        y += 1;
    });

    // println!("{:?}", point_map);

    // For all values in map get the vector of points.
    // For each point in vector, find the slope and distance.
    // If distance is not more than 2, discard.
    // If distance is more than 2, calc the 2 points on either end on the same slope.

    for (key, value) in point_map.into_iter() {
        // println!("KEY TESTING: {}", key);
        let vect = &value;

        let mut index = 0;
        while index < vect.len() {
            let mut test_index = index + 1;
            while test_index < vect.len() {
                let p1 = vect[index];
                let p2 = vect[test_index];

                // println!("TEST {:?} {:?}", p1, p2);

                let distance = get_distance(p1, p2);

                if distance > 1 as f64 {
                    let new_points = test_points(p1, p2);

                    let new_point1 = new_points.0;
                    let new_point2 = new_points.1;

                    let myp1 = MyPoint{x: new_point1.x() as i32, y: new_point1.y() as i32};
                    let myp2 = MyPoint{x: new_point2.x() as i32, y: new_point2.y() as i32};

                    if is_in_grid(new_point1, line_width.try_into().unwrap(), line_count.try_into().unwrap()) {
                        antennas.insert(myp1);
                    } 

                    if is_in_grid(new_point2, line_width.try_into().unwrap(), line_count.try_into().unwrap()) {
                        antennas.insert(myp2);
                    }
                }

                test_index += 1;
            }
            index += 1;
        }
    }
    println!("{}", antennas.len());
}

fn is_in_grid(point: Point, rows: i32, cols: i32) -> bool {
    let r = rows - 1;
    let c = cols - 1;

    if point.x() < 0.0 || point.x() > c.into() {
        return false;
    }

    if point.y() < 0.0 || point.y() > r.into() {
        return false;
    }

    // println!("{:?}", point);
    return true;
}

fn test_points(point1: Point, point2: Point) -> (Point, Point) {
    let slope = get_slope(point1, point2);

    let newPoint1 = Point::new(point1.x() + slope.0, point1.y() + slope.1);
    let newPoint2 = Point::new(point2.x() - slope.0, point2.y() - slope.1);

    (newPoint1, newPoint2)
}

fn get_distance(point1: Point, point2: Point) -> f64 {
    let diff_x = point1.x() - point2.x();
    let diff_y = point1.y() - point2.y();

    let sq_x = diff_x * diff_x;
    let sq_y = diff_y * diff_y;

    let distance = f64::sqrt(sq_x + sq_y);

    distance
}

fn get_slope(point1: Point, point2: Point) -> (f64, f64) {
    let x = point1.x() - point2.x();
    let y = point1.y() - point2.y();

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_line_test() {
        let mut point1 = Point::new(5 as f64, 7 as f64);
        let mut point2 = Point::new(9 as f64, 7 as f64);
        assert_eq!(test_in_line(point1, point2), true);
        
        point1 = Point::new(5 as f64, 7 as f64);
        point2 = Point::new(5 as f64, 9 as f64);
        assert_eq!(test_in_line(point1, point2), true);
        
        point1 = Point::new(5 as f64, 3 as f64);
        point2 = Point::new(1 as f64, 9 as f64);
        assert_eq!(test_in_line(point1, point2), false);
        
        point1 = Point::new(4 as f64, 3 as f64);
        point2 = Point::new(5 as f64, 5 as f64);
        assert_eq!(test_in_line(point1, point2), true);
    }
}