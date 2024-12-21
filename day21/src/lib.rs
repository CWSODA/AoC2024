use std::{collections::HashMap, usize};

pub fn solve(input: &str) -> (usize, usize) {
    let codes: Vec<_> = input.lines().collect();

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let mut num_cache: HashMap<(char, char, usize), usize> = HashMap::new();
    let mut dir_cache: HashMap<(Button, Button, usize), usize> = HashMap::new();
    for code in codes {
        // find value of the code, removes trailing 'A' before parse
        let mut number = code.to_string();
        number.pop();
        let number = number.parse::<usize>().unwrap();

        p1_sum += get_shortest(code, 2, &mut num_cache, &mut dir_cache) * number;
        p2_sum += get_shortest(code, 25, &mut num_cache, &mut dir_cache) * number;
    }

    (p1_sum, p2_sum)
}

fn get_shortest(
    code: &str,
    level: usize,
    num_cache: &mut HashMap<(char, char, usize), usize>,
    dir_cache: &mut HashMap<(Button, Button, usize), usize>,
) -> usize {
    // cache of (from, to, level), minimum buttons in last
    // level 1 is the human dirpad

    let mut sum = shortest_numpad(
        'A',
        code.chars().next().unwrap(),
        level,
        num_cache,
        dir_cache,
    );

    for (from, to) in code.chars().zip(code.chars().skip(1)) {
        sum += shortest_numpad(from, to, level, num_cache, dir_cache);
    }

    sum
}

// A       0   2       9       A
// A   <   A ^ A >  ^^ A  vvv  A
// Av<<A>>^A<A>AvA<^AA>A<vAAA>^A

fn shortest_dirpad(
    from: &Button,
    to: &Button,
    level: usize,
    cache: &mut HashMap<(Button, Button, usize), usize>,
) -> usize {
    if from == to {
        return 1;
    }

    let key = (*from, *to, level);
    if let Some(&cost) = cache.get(&key) {
        return cost;
    }
    if level == 1 {
        let cost = dirpad_move(from, to)[0].len();
        cache.insert(key, cost);

        return cost;
    }

    // find cost
    let mut min_cost = usize::MAX;
    for possible in dirpad_move(from, to) {
        let mut current_cost = shortest_dirpad(&Button::Press, &possible[0], level - 1, cache);

        for (button1, button2) in possible.iter().zip(possible.iter().skip(1)) {
            current_cost += shortest_dirpad(button1, button2, level - 1, cache);
        }

        if current_cost < min_cost {
            min_cost = current_cost;
        }
    }

    cache.insert(key, min_cost);
    min_cost
}

fn shortest_numpad(
    from: char,
    to: char,
    level: usize,
    num_cache: &mut HashMap<(char, char, usize), usize>,
    dir_cache: &mut HashMap<(Button, Button, usize), usize>,
) -> usize {
    if from == to {
        return 1;
    }

    let key = (from, to, level);
    if let Some(&cost) = num_cache.get(&key) {
        return cost;
    }

    // find cost
    let mut min_cost = usize::MAX;
    for possible in numpad_move(from, to) {
        let mut current_cost = shortest_dirpad(&Button::Press, &possible[0], level, dir_cache);

        for (button1, button2) in possible.iter().zip(possible.iter().skip(1)) {
            current_cost += shortest_dirpad(button1, button2, level, dir_cache);
        }

        if current_cost < min_cost {
            min_cost = current_cost;
        }
    }

    num_cache.insert(key, min_cost);
    min_cost
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
