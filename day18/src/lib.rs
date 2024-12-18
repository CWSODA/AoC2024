use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn solve(input: &str, grid_size: usize, bytes: usize) -> (u32, Vec2) {
    let corrupted = parse(input);

    let p1_steps = *walk(&corrupted[0..bytes], grid_size)
        .get(&Vec2::new(grid_size, grid_size))
        .unwrap();

    let mut exit_blocker = None;
    // bytes for part1 is guarenteed to work
    for byte in bytes..corrupted.len() {
        if !has_exit(&corrupted[0..=byte], grid_size) {
            exit_blocker = Some(corrupted[byte]);
            break;
        }
    }

    (p1_steps, exit_blocker.expect("No byte blocks the exit!"))
}

fn walk(corrupted: &[Vec2], grid_size: usize) -> HashMap<Vec2, u32> {
    let start_pos = Vec2::new(0, 0);
    let end_pos = Vec2::new(grid_size, grid_size);
    let corrupted: HashSet<_> = corrupted.iter().map(|v| *v).collect();
    let mut costs = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_pos)));

    while let Some(Reverse((last_cost, last_pos))) = queue.pop() {
        for pos in last_pos.around(grid_size) {
            if !corrupted.contains(&pos) {
                let cost = last_cost + 1;

                if let Some(prev_cost) = costs.get(&pos) {
                    // if previous cost is equal or better, dont continue walking
                    if *prev_cost <= cost {
                        continue;
                    }
                }

                // if prev_cost is worst or does not exist, continue walking
                costs.insert(pos, cost);

                if !(pos == end_pos) {
                    queue.push(Reverse((cost, pos)));
                }
            }
        }
    }

    costs
}

fn has_exit(corrupted: &[Vec2], grid_size: usize) -> bool {
    let start_pos = Vec2::new(0, 0);
    let end_pos = Vec2::new(grid_size, grid_size);
    let mut seen = HashSet::<Vec2>::from([start_pos]);
    let corrupted = corrupted.iter().map(|v| *v).collect();

    fill(&corrupted, &mut seen, &start_pos, &end_pos, grid_size)
}

fn fill(
    corrupted: &HashSet<Vec2>,
    seen: &mut HashSet<Vec2>,
    last_pos: &Vec2,
    end_pos: &Vec2,
    grid_size: usize,
) -> bool {
    for pos in last_pos.around(grid_size) {
        if pos == *end_pos {
            seen.insert(*end_pos);
            return true;
        }
        if !corrupted.contains(&pos) {
            if seen.insert(pos) {
                // if newly inserted position, fill again
                if fill(corrupted, seen, &pos, end_pos, grid_size) {
                    return true;
                }
            }
        }
    }

    false
}

fn parse(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split(|ch: char| !ch.is_numeric())
                .filter(|slice| !slice.is_empty())
                .map(|slice| slice.parse().unwrap());
            Vec2::new(nums.next().unwrap(), nums.next().unwrap())
        })
        .collect()
}

// for debug
fn _print_grid(corrupted: &[Vec2], seen: &HashSet<Vec2>, grid_size: usize) {
    for y in 0..=grid_size {
        for x in 0..=grid_size {
            let pos = Vec2::new(x, y);
            print!(
                "{}",
                if corrupted.contains(&pos) {
                    '#'
                } else {
                    if seen.contains(&pos) {
                        'O'
                    } else {
                        '.'
                    }
                }
            )
        }
        println!()
    }
    println!()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }

    fn around(&self, grid_size: usize) -> Vec<Vec2> {
        let mut output = vec![];

        if self.x != 0 {
            output.push(Vec2::new(self.x - 1, self.y));
        }

        if self.x < grid_size {
            output.push(Vec2::new(self.x + 1, self.y));
        }

        if self.y != 0 {
            output.push(Vec2::new(self.x, self.y - 1));
        }

        if self.y < grid_size {
            output.push(Vec2::new(self.x, self.y + 1));
        }

        output
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
