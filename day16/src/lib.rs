use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    u32,
};

pub fn solve(input: &str) -> (u32, usize) {
    let (grid, (start_pos, end_pos)) = parse(input);

    // hashmap of (pos, direction), (cost)
    let mut costs = HashMap::new();
    costs.insert((start_pos, Dir::East), 0);

    walk(&grid, &mut costs, start_pos, end_pos);

    let mut min_score = u32::MAX;
    let mut end_dirs = vec![];
    for dir in Dir::all() {
        if let Some(score) = costs.get(&(end_pos, dir)) {
            if *score < min_score {
                min_score = *score;
                end_dirs = vec![dir];
            } else if *score == min_score {
                end_dirs.push(dir);
            }
        }
    }

    let visited = back_track(&mut costs, start_pos, end_pos, min_score, end_dirs);
    _print_visited(&grid, &visited);

    (min_score, visited.len())
}

fn walk(grid: &[Vec<Cell>], costs: &mut HashMap<(Vec2, Dir), u32>, start_pos: Vec2, end_pos: Vec2) {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_pos, Dir::East)));

    while let Some(Reverse((last_cost, last_pos, last_dir))) = queue.pop() {
        for (dir, pos) in last_pos.next_cells(last_dir) {
            if grid[pos.y][pos.x] == Cell::Path {
                let cost = last_cost + 1 + if dir == last_dir { 0 } else { 1000 };

                if let Some(prev_cost) = costs.get(&(pos, dir)) {
                    // if previous cost is equal or better, dont continue walking
                    if *prev_cost <= cost {
                        continue;
                    }
                }

                // if prev_cost is worst or does not exist, continue walking
                costs.insert((pos, dir), cost);

                if !(pos == end_pos) {
                    queue.push(Reverse((cost, pos, dir)));
                }
            }
        }
    }
}

fn back_track(
    costs: &mut HashMap<(Vec2, Dir), u32>,
    start_pos: Vec2,
    end_pos: Vec2,
    min_score: u32,
    end_dirs: Vec<Dir>,
) -> HashSet<Vec2> {
    let mut visited = HashSet::from([end_pos]);

    let mut queue: Vec<_> = end_dirs
        .into_iter()
        .map(|dir| (end_pos, min_score, dir))
        .collect();
    while let Some((pos, last_cost, forward_dir)) = queue.pop() {
        let back_pos = pos.step_back(forward_dir);
        for dir in Dir::all() {
            if let Some(cost) = costs.get(&(back_pos, dir)) {
                if *cost < last_cost {
                    visited.insert(back_pos);

                    if back_pos != start_pos {
                        queue.push((back_pos, *cost, dir));
                    }
                }
            }
        }
    }

    visited
}

// returns maze grid and start/end positions
fn parse(input: &str) -> (Vec<Vec<Cell>>, (Vec2, Vec2)) {
    let mut start_pos = None;
    let mut end_pos = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Path,
                    'S' => {
                        start_pos = Some(Vec2::new(x, y));
                        Cell::Path
                    }
                    'E' => {
                        end_pos = Some(Vec2::new(x, y));
                        Cell::Path
                    }
                    _ => panic!("unrecognized char"),
                })
                .collect()
        })
        .collect();

    (
        grid,
        (
            start_pos.expect("no start position"),
            end_pos.expect("no end position"),
        ),
    )
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cell {
    Wall,
    Path,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }

    // returns next possible path, excludes direction the reindeer came from
    fn next_cells(&self, dir: Dir) -> [(Dir, Vec2); 3] {
        match dir {
            Dir::North => [
                (Dir::North, Vec2::new(self.x, self.y - 1)),
                (Dir::East, Vec2::new(self.x + 1, self.y)),
                (Dir::West, Vec2::new(self.x - 1, self.y)),
            ],
            Dir::South => [
                (Dir::South, Vec2::new(self.x, self.y + 1)),
                (Dir::East, Vec2::new(self.x + 1, self.y)),
                (Dir::West, Vec2::new(self.x - 1, self.y)),
            ],
            Dir::East => [
                (Dir::North, Vec2::new(self.x, self.y - 1)),
                (Dir::South, Vec2::new(self.x, self.y + 1)),
                (Dir::East, Vec2::new(self.x + 1, self.y)),
            ],
            Dir::West => [
                (Dir::North, Vec2::new(self.x, self.y - 1)),
                (Dir::South, Vec2::new(self.x, self.y + 1)),
                (Dir::West, Vec2::new(self.x - 1, self.y)),
            ],
        }
    }

    fn step_back(&self, dir: Dir) -> Self {
        Vec2::new(
            match dir {
                Dir::North | Dir::South => self.x,
                Dir::East => self.x - 1,
                Dir::West => self.x + 1,
            },
            match dir {
                Dir::North => self.y + 1,
                Dir::South => self.y - 1,
                Dir::East | Dir::West => self.y,
            },
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn all() -> [Dir; 4] {
        [Dir::North, Dir::South, Dir::East, Dir::West]
    }
}

// for debug
fn _print_visited(grid: &[Vec<Cell>], visited: &HashSet<Vec2>) {
    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            match cell {
                Cell::Wall => print!("#"),
                Cell::Path => print!(
                    "{}",
                    if visited.contains(&Vec2::new(x, y)) {
                        'O'
                    } else {
                        '.'
                    }
                ),
            }
        }
        println!();
    }
}
