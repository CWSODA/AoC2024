use std::collections::HashSet;

pub fn solve(input: &str) -> (usize, usize) {
    let (grid1, grid2, moves, start_pos1, start_pos2) = parse(input);
    (
        part1(grid1, &moves, start_pos1),
        part2(grid2, &moves, start_pos2),
    )
}

fn part1(mut grid: Vec<Vec<State>>, moves: &[Dir], start_pos: Vec2) -> usize {
    let mut bot_pos = start_pos;

    for dir in moves {
        if shift(&mut grid, *dir, bot_pos) {
            bot_pos = bot_pos.shifted(*dir);
        }
    }

    calc_gps(&grid)
}

fn part2(mut grid: Vec<Vec<State>>, moves: &[Dir], start_pos: Vec2) -> usize {
    let mut bot_pos = start_pos;

    for dir in moves {
        if shift(&mut grid, *dir, bot_pos) {
            bot_pos = bot_pos.shifted(*dir);
        }
    }

    calc_gps(&grid)
}

fn shift(grid: &mut Vec<Vec<State>>, dir: Dir, pos: Vec2) -> bool {
    let new_pos = pos.shifted(dir);

    match grid.get(new_pos) {
        State::Empty => { /* Allow Move */ }
        State::Box => {
            if !shift(grid, dir, new_pos) {
                return false;
            }
        }
        State::Wall => return false,
        State::Bot => {
            panic!("should not encounter bot at {:?}", new_pos)
        }
        // part2 adds new box type
        // if movement is vertical, use the shift_large function
        // if movement is horizontal then shift regularly
        State::BoxL => {
            if dir.is_vert() {
                if !shift_large(grid, dir, HashSet::from([new_pos, new_pos.right()])) {
                    return false;
                }
            } else {
                if !shift(grid, dir, new_pos) {
                    return false;
                }
            }
        }
        State::BoxR => {
            if dir.is_vert() {
                if !shift_large(grid, dir, HashSet::from([new_pos, new_pos.left()])) {
                    return false;
                }
            } else {
                if !shift(grid, dir, new_pos) {
                    return false;
                }
            }
        }
    }

    grid.move_to(pos, new_pos);
    true
}

fn shift_large(grid: &mut Vec<Vec<State>>, dir: Dir, to_shift: HashSet<Vec2>) -> bool {
    assert!(dir.is_vert());
    let mut to_shift_next = HashSet::new();

    for pos in &to_shift {
        match grid.get(pos.shifted(dir)) {
            State::Wall => return false,
            State::Empty => { /* Allow Move */ }
            State::BoxL => {
                to_shift_next.insert(pos.shifted(dir));
                to_shift_next.insert(pos.right().shifted(dir));
            }
            State::BoxR => {
                to_shift_next.insert(pos.shifted(dir));
                to_shift_next.insert(pos.left().shifted(dir));
            }
            _ => panic!("Should never encounter box or bot"),
        }
    }

    // check if further moves are valid
    if !to_shift_next.is_empty() && !shift_large(grid, dir, to_shift_next) {
        return false;
    }

    for pos in to_shift {
        grid.move_to(pos, pos.shifted(dir));
    }

    true
}

fn calc_gps(grid: &[Vec<State>]) -> usize {
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, state) in line.iter().enumerate() {
            if *state == State::Box {
                sum += 100 * y + x;
            } else if *state == State::BoxL {
                sum += 100 * y + x;
            }
        }
    }

    sum
}

// returns grid_p1, grid_p2, moves, start_pos_1, start_pos_2
fn parse(input: &str) -> (Vec<Vec<State>>, Vec<Vec<State>>, Vec<Dir>, Vec2, Vec2) {
    let mut grid_p1: Vec<Vec<State>> = vec![];
    let mut grid_p2: Vec<Vec<State>> = vec![];
    let mut moves: Vec<Dir> = vec![];
    let mut bot_pos_1 = Vec2::new(0, 0);
    let mut bot_pos_2 = Vec2::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        // all grids start and end with # wall
        if line.starts_with('#') {
            let mut current_line1 = vec![];
            let mut current_line2 = vec![];
            for (x, ch) in line.chars().enumerate() {
                current_line1.push(match ch {
                    '#' => State::Wall,
                    '@' => {
                        bot_pos_1 = Vec2::new(x, y);
                        State::Bot
                    }
                    'O' => State::Box,
                    _ => State::Empty,
                });
                match ch {
                    '#' => {
                        current_line2.push(State::Wall);
                        current_line2.push(State::Wall);
                    }
                    '@' => {
                        bot_pos_2 = Vec2::new(2 * x, y);
                        current_line2.push(State::Bot);
                        current_line2.push(State::Empty);
                    }
                    'O' => {
                        current_line2.push(State::BoxL);
                        current_line2.push(State::BoxR);
                    }
                    _ => {
                        current_line2.push(State::Empty);
                        current_line2.push(State::Empty);
                    }
                };
            }
            grid_p1.push(current_line1);
            grid_p2.push(current_line2);
        } else {
            for ch in line.chars() {
                match ch {
                    '<' => moves.push(Dir::Left),
                    '>' => moves.push(Dir::Right),
                    '^' => moves.push(Dir::Up),
                    'v' => moves.push(Dir::Down),
                    _ => { /* Do Nothing */ }
                };
            }
        }
    }

    (grid_p1, grid_p2, moves, bot_pos_1, bot_pos_2)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Bot,
    Box,
    Wall,
    Empty,
    BoxL,
    BoxR,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn is_vert(&self) -> bool {
        *self == Dir::Up || *self == Dir::Down
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }

    // should never be out of bounds since walls dont move
    fn shifted(&self, m: Dir) -> Self {
        match m {
            Dir::Up => Vec2::new(self.x, self.y - 1),
            Dir::Down => Vec2::new(self.x, self.y + 1),
            Dir::Left => Vec2::new(self.x - 1, self.y),
            Dir::Right => Vec2::new(self.x + 1, self.y),
        }
    }

    fn left(&self) -> Self {
        Vec2::new(self.x - 1, self.y)
    }
    fn right(&self) -> Self {
        Vec2::new(self.x + 1, self.y)
    }
}

trait Grid {
    fn get(&self, pos: Vec2) -> State;
    fn get_mut(&mut self, pos: Vec2) -> &mut State;
    fn move_to(&mut self, pos: Vec2, new_pos: Vec2);
}

impl Grid for Vec<Vec<State>> {
    fn get(&self, pos: Vec2) -> State {
        self[pos.y][pos.x]
    }
    fn get_mut(&mut self, pos: Vec2) -> &mut State {
        &mut self[pos.y][pos.x]
    }
    fn move_to(&mut self, pos: Vec2, new_pos: Vec2) {
        *self.get_mut(new_pos) = self.get(pos);
        *self.get_mut(pos) = State::Empty;
    }
}

// for debug
fn _print_grid(grid: &[Vec<State>]) {
    for line in grid {
        for state in line {
            print!(
                "{}",
                match state {
                    State::Bot => '@',
                    State::Box => 'O',
                    State::Wall => '#',
                    State::Empty => '.',
                    State::BoxL => '[',
                    State::BoxR => ']',
                }
            );
        }
        println!();
    }
}
