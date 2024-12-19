use cgmath::Point2;

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Game {
    button_a: Point2<i64>,
    button_b: Point2<i64>,
    prize:   Point2<i64>
}

fn main() {
    let count = INPUT.lines().count();

    let mut line_index = 0;
    
    let mut games : Vec<Game> = Vec::new();

    while line_index < count {
        let l1 = INPUT.lines().nth(line_index);
        let l2 = INPUT.lines().nth(line_index + 1);
        let l3 = INPUT.lines().nth(line_index + 2);

        let game = parse_game(l1.expect("REASON"), l2.expect("REASON"), l3.expect("REASON"));
        games.push(game);

        line_index += 4;
    }

    let mut total = 0;
    for g in games {
        total += process_game(g.clone());
    }

    println!("{}", total);
}

fn process_game(game: Game) -> i64 {
    let mut total = 0;

    let new_x = game.prize.x * game.button_a.y;
    let new_y = game.prize.y * game.button_a.x;

    let diff = new_x - new_y;

    let mult_1 = game.button_a.y * game.button_b.x;
    let mult_2 = game.button_a.x * game.button_b.y;

    let diff_2 = mult_1 - mult_2;

    let b_answer = diff / diff_2;
    let remain = diff % diff_2;

    if remain == 0 {
        let aa = game.button_a.y * game.button_b.x * b_answer;

        let bb = (game.prize.x * game.button_a.y) - aa;

        let a_answer = bb / (game.button_a.y * game.button_a.x);
        total = (3 * a_answer) + (1 * b_answer);
        println!("A: {} and B: {} Total: {}", b_answer, a_answer, total);
    }
    
    total.try_into().unwrap()
}

fn parse_game(line_a: &str, line_b: &str, line_prize: &str) -> Game {
    let a_button = parse_button(line_a);
    let b_button = parse_button(line_b);
    let prize = parse_prize(line_prize);

    Game{button_a: a_button, button_b: b_button, prize: prize}
}

fn parse_prize(line: &str) -> Point2<i64> {
    let x_index = line.find("X=").unwrap() + 1;
    let y_index = line.find("Y=").unwrap() + 1;

    let x_str = &line[x_index + 1..y_index - 3];
    let y_str = &line[y_index + 1 ..line.len()];

    Point2{x: x_str.parse::<i64>().expect("REASON") + 10000000000000, y: y_str.parse::<i64>().expect("REASON") + 10000000000000}
}

fn parse_button(line: &str) -> Point2<i64> {
    let x_index = line.find("X+").unwrap() + 1;
    let y_index = line.find("Y+").unwrap() + 1;

    let x_str = &line[x_index + 1..y_index - 3];
    let y_str = &line[y_index + 1..line.len()];

    Point2{x: x_str.parse::<i64>().expect("REASON"), y: y_str.parse::<i64>().expect("REASON")}
}