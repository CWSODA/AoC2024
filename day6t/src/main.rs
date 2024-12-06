fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

fn solve(input: &str) -> (i32, i32) {
    let mut grid = vec![];
    let cur_pos = parse(input, &mut grid);
    let mut placeables = vec![];

    let p1 = part1(&grid, cur_pos, &mut placeables);
    let p2 = part2(&grid, cur_pos, &placeables);

    return (p1, p2);
}

fn part1(grid: &Vec<Vec<State>>, start_pos: (i32, i32), placeables: &mut Vec<(i32, i32)>) -> i32 {
    let mut passed = 0;
    let mut cur_pos = start_pos;
    let mut next_pos;
    let mut cur_dir = Direction::N;
    let mut grid = grid.clone();

    loop {
        match cur_dir {
            // x, y
            Direction::N => next_pos = (cur_pos.0, cur_pos.1 - 1),
            Direction::S => next_pos = (cur_pos.0, cur_pos.1 + 1),
            Direction::E => next_pos = (cur_pos.0 + 1, cur_pos.1),
            Direction::W => next_pos = (cur_pos.0 - 1, cur_pos.1),
        }

        // check next block
        match get_grid(&mut grid, next_pos) {
            Some(state) => match state {
                State::Obs => cur_dir = turn(cur_dir),
                State::Passed(_) | State::Normal => {
                    cur_pos = next_pos;
                    set_state(&mut grid, next_pos, State::Passed((0, 0)));
                }
            },
            None => break, // out of bounds
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, state) in row.iter().enumerate() {
            match state {
                State::Passed(_) => {
                    passed += 1;
                    if (x as i32, y as i32) != start_pos {
                        placeables.push((x as i32, y as i32));
                    }
                }
                _ => {}
            }
        }
    }

    passed
}

fn part2(grid: &Vec<Vec<State>>, start_pos: (i32, i32), placeables: &[(i32, i32)]) -> i32 {
    let mut solutions = 0;

    'alt_uni: for new_obs in placeables {
        let mut temp = grid.clone();
        // adds new obstacle
        set_state(&mut temp, *new_obs, State::Obs);
        let mut cur_pos = start_pos;

        // will loop check
        let mut next_pos;
        let mut cur_dir = Direction::N;

        loop {
            match cur_dir {
                // x, y
                Direction::N => next_pos = (cur_pos.0, cur_pos.1 - 1),
                Direction::S => next_pos = (cur_pos.0, cur_pos.1 + 1),
                Direction::E => next_pos = (cur_pos.0 + 1, cur_pos.1),
                Direction::W => next_pos = (cur_pos.0 - 1, cur_pos.1),
            }

            // check next block
            match get_grid(&mut temp, next_pos) {
                Some(state) => match state {
                    State::Obs => cur_dir = turn(cur_dir),
                    State::Passed(mut count) => {
                        if cur_dir.is_vert() {
                            count.1 += 1;
                        } else {
                            count.0 += 1; // else horizontal
                        }

                        if count.0 > 2 || count.1 > 2 {
                            solutions += 1;
                            continue 'alt_uni;
                        }

                        cur_pos = next_pos;
                        set_state(&mut temp, next_pos, State::Passed(count))
                    }
                    State::Normal => {
                        cur_pos = next_pos;
                        set_state(
                            &mut temp,
                            next_pos,
                            State::Passed(match cur_dir {
                                Direction::N | Direction::S => (0, 1),
                                Direction::E | Direction::W => (1, 0),
                            }),
                        )
                    }
                },
                None => continue 'alt_uni, // out of bounds, not a loop
            }
        }
    }

    solutions
}

fn parse(input: &str, grid: &mut Vec<Vec<State>>) -> (i32, i32) {
    grid.clear();
    let mut guard_pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut v = vec![];
        for (x, c) in line.chars().enumerate() {
            v.push(match c {
                '.' => State::Normal,
                '#' => State::Obs,
                '^' => {
                    guard_pos = (x, y);
                    // starts north so passed once on vertical
                    State::Passed((0, 1))
                }
                _ => panic!(),
            });
        }
        grid.push(v);
    }

    (guard_pos.0 as i32, guard_pos.1 as i32)
}

fn get_grid(grid: &mut Vec<Vec<State>>, pos: (i32, i32)) -> Option<State> {
    if pos.0 < 0 || pos.1 < 0 {
        return None; // out of bounds
    }

    if let Some(v) = grid.get(pos.1 as usize) {
        if let Some(state) = v.get(pos.0 as usize) {
            return Some(*state);
        }
    }

    return None;
}

// here so i dont forget the outer vec is the line vector(is the y direction)
fn set_state(grid: &mut Vec<Vec<State>>, pos: (i32, i32), new_state: State) {
    grid[pos.1 as usize][pos.0 as usize] = new_state;
}

fn turn(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Obs,
    // times it was passed in horizontal and vertical
    Passed((i32, i32)),
    Normal,
}

enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn is_vert(&self) -> bool {
        match self {
            Direction::N | Direction::S => true,
            _ => false,
        }
    }
}

// for debugging
fn _print_grid(grid: &Vec<Vec<State>>) {
    println!();
    for row in grid {
        println!();
        for state in row {
            print!(
                "{}",
                match state {
                    State::Obs => '#',
                    State::Passed(count) => {
                        match count {
                            (0, _) => '|',
                            (_, 0) => '-',
                            _ => '+',
                        }
                    }
                    State::Normal => '.',
                }
            );
        }
    }
    println!();
}
