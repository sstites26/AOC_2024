use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct MyPoint {
    x: i32,
    y: i32,
}

fn main() {
    let line_count = INPUT.lines().count();
    let line_width = INPUT.lines().next().expect("DOESNT EXIST").len();

    let mut point_map: HashMap<char, Vec<MyPoint>> = HashMap::new();
    let mut antennas : HashSet<MyPoint> = HashSet::new();
    
    let mut y = 0;
    INPUT.lines().for_each(|line| {
        let mut x = 0;

        for c in line.chars() {
            if c != '.' && c != '#' {
                let point = MyPoint{x: x, y: y};

                if point_map.contains_key(&c) {
                    point_map.get_mut(&c).expect("REASON").push(point);
                } else {
                    let mut vect : Vec<MyPoint> = Vec::new();
                    vect.push(point);
                    point_map.insert(c, vect);
                }
            }

            x += 1;
        }

        y += 1;
    });

    for (key, value) in point_map.clone().into_iter() {
        let vect = &value;

        let mut index = 0;
        while index < vect.len() {
            let mut test_index = index + 1;
            while test_index < vect.len() {
                let p1 = vect[index];
                let p2 = vect[test_index];

                let distance = get_distance(p1, p2);

                let all_the_new_points = test_points(p1, p2, line_width.try_into().unwrap(), line_count.try_into().unwrap());
                antennas.extend(all_the_new_points.clone());
                test_index += 1;
            }
            antennas.insert(vect[index]);
            index += 1;
        }
    }

    antennas.iter().for_each(|a| println!("{:?}", a));
    println!("{}", antennas.len());
}

fn is_in_grid(point: MyPoint, rows: i32, cols: i32) -> bool {
    let r = rows - 1;
    let c = cols - 1;

    if point.x < 0 || point.x > c.into() {
        return false;
    }

    if point.y < 0 || point.y > r.into() {
        return false;
    }

    return true;
}

fn test_points(point1: MyPoint, point2: MyPoint, cols: i32, rows: i32) -> HashSet<MyPoint> {
    let slope = get_slope(point1, point2);

    let mut new_points : HashSet<MyPoint> = HashSet::new();

    if slope.0 == 0 && slope.1 == 0 {
        println!("BAD");
    } else {
        
        let mut point_in_bounds = true;

        let mut test_point = MyPoint{x: point1.x, y: point1.y};
        while point_in_bounds {
            let newX = test_point.x + slope.0;
            let newY = test_point.y + slope.1;

            test_point = MyPoint{x: newX as i32, y: newY as i32};

            if is_in_grid(test_point, rows, cols) {
                new_points.insert(test_point);
            } else {
                point_in_bounds = false;
            }
        }

        point_in_bounds = true;
        test_point = MyPoint{x: point2.x, y: point2.y};
        while point_in_bounds {
            let newX = test_point.x - slope.0;
            let newY = test_point.y - slope.1;

            test_point = MyPoint{x: newX, y: newY};

            if is_in_grid(test_point, rows, cols) {
                new_points.insert(test_point);
            } else {
                point_in_bounds = false;
            }
        }
    }

    new_points
}

fn get_distance(point1: MyPoint, point2: MyPoint) -> i32 {
    let diff_x = point1.x - point2.x;
    let diff_y = point1.y - point2.y;

    let sq_x = diff_x * diff_x;
    let sq_y = diff_y * diff_y;

    let distance = f64::sqrt((sq_x + sq_y).into());

    distance as i32
}

fn get_slope(point1: MyPoint, point2: MyPoint) -> (i32, i32) {
    let x = point1.x - point2.x;
    let y = point1.y - point2.y;

    (x, y)
}