pub fn solve(input: &str) -> (usize, usize) {
    let codes: Vec<_> = input.lines().collect();

    let mut sum = 0;
    for code in codes {
        let mut number = code.to_string();
        number.pop();

        let shortest = get_shortest(code);
        println!("Shortest for code {code} is {shortest}");
        sum += number.parse::<usize>().unwrap() * shortest;
    }

    (sum, 0)
}

fn get_shortest(code: &str) -> usize {
    // num <- dir <- dir <- human_dir
    // bot starts from button A
    let code = "A".to_string() + code;

    let mut dirbot_1: Vec<Vec<Button>> = vec![vec![]];
    for (from, to) in code.chars().zip(code.chars().skip(1)) {
        let mut temp = vec![];

        for numpad_1 in numpad_move(from, to) {
            for pos in &dirbot_1 {
                let mut current = pos.clone();
                current.extend_from_slice(&numpad_1);
                temp.push(current);
            }
        }

        dirbot_1 = temp;
    }

    let mut dirbot_2 = vec![];
    for possibles in &dirbot_1 {
        dirbot_2.append(&mut get_possibles(possibles));
    }

    let mut human = vec![];
    for possibles in &dirbot_2 {
        human.append(&mut get_possibles(possibles));
    }

    let mut min = human[0].len();
    for possible in human {
        if possible.len() < min {
            min = possible.len()
        }
    }

    min
}

fn get_possibles(input: &[Button]) -> Vec<Vec<Button>> {
    // starts from A button
    let mut possibles: Vec<Vec<Button>> = dirpad_move(&Button::Press, &input[0]);

    for (from, to) in input.iter().zip(input.iter().skip(1)) {
        let mut temp = vec![];

        for next_bot in dirpad_move(from, to) {
            for pos in &possibles {
                let mut current = pos.clone();
                current.extend_from_slice(&next_bot);
                temp.push(current);
            }
        }

        possibles = temp;
    }

    possibles
}

fn numpad_move(from: char, to: char) -> Vec<Vec<Button>> {
    if from == to {
        return vec![vec![Button::Press]];
    }

    let pos1 = get_num_pos(from);
    let pos2 = get_num_pos(to);

    let horz = vec![
        if pos1.x > pos2.x {
            Button::Left
        } else {
            Button::Right
        };
        (pos1.x as i32 - pos2.x as i32).abs() as usize
    ];
    let vert = vec![
        if pos1.y > pos2.y {
            Button::Up
        } else {
            Button::Down
        };
        (pos1.y as i32 - pos2.y as i32).abs() as usize
    ];

    let mut moves = vec![];
    if !horz.is_empty() && !(pos1.y == 3 && pos2.x == 0) {
        let mut vec1 = horz.clone();
        vec1.extend_from_slice(&vert);
        vec1.push(Button::Press);

        moves.push(vec1);
    }
    if !vert.is_empty() && !(pos2.y == 3 && pos1.x == 0) {
        let mut vec2 = vert.clone();
        vec2.extend_from_slice(&horz);
        vec2.push(Button::Press);

        moves.push(vec2);
    }

    moves
}

fn dirpad_move(from: &Button, to: &Button) -> Vec<Vec<Button>> {
    if from == to {
        return vec![vec![Button::Press]];
    }

    let pos1 = get_dir_pos(from);
    let pos2 = get_dir_pos(to);

    let horz = vec![
        if pos1.x > pos2.x {
            Button::Left
        } else {
            Button::Right
        };
        (pos1.x as i32 - pos2.x as i32).abs() as usize
    ];
    let vert = vec![
        if pos1.y > pos2.y {
            Button::Up
        } else {
            Button::Down
        };
        (pos1.y as i32 - pos2.y as i32).abs() as usize
    ];

    let mut moves = vec![];
    if !horz.is_empty() && !(pos1.y == 0 && pos2.x == 0) {
        let mut vec1 = horz.clone();
        vec1.extend_from_slice(&vert);
        vec1.push(Button::Press);

        moves.push(vec1);
    }
    if !vert.is_empty() && !(pos1.x == 0 && pos2.y == 0) {
        let mut vec2 = vert.clone();
        vec2.extend_from_slice(&horz);
        vec2.push(Button::Press);

        moves.push(vec2);
    }

    moves
}

struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Button {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Button::Up => '^',
                Button::Down => 'v',
                Button::Left => '<',
                Button::Right => '>',
                Button::Press => 'A',
            }
        )
    }
}

fn get_dir_pos(dir: &Button) -> Vec2 {
    match dir {
        Button::Up => Vec2 { x: 1, y: 0 },
        Button::Down => Vec2 { x: 1, y: 1 },
        Button::Left => Vec2 { x: 0, y: 1 },
        Button::Right => Vec2 { x: 2, y: 1 },
        Button::Press => Vec2 { x: 2, y: 0 },
    }
}

fn get_num_pos(num: char) -> Vec2 {
    match num {
        '9' => Vec2 { x: 2, y: 0 },
        '8' => Vec2 { x: 1, y: 0 },
        '7' => Vec2 { x: 0, y: 0 },
        '6' => Vec2 { x: 2, y: 1 },
        '5' => Vec2 { x: 1, y: 1 },
        '4' => Vec2 { x: 0, y: 1 },
        '3' => Vec2 { x: 2, y: 2 },
        '2' => Vec2 { x: 1, y: 2 },
        '1' => Vec2 { x: 0, y: 2 },
        '0' => Vec2 { x: 1, y: 3 },
        'A' => Vec2 { x: 2, y: 3 },
        _ => panic!("Invalid numpad input!"),
    }
}

// for debug
fn _print_possible_buttons(input: &[Vec<Button>]) {
    for possibility in input {
        for button in possibility {
            print!("{button}");
        }
        println!();
    }
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
