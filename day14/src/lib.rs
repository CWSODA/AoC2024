use std::collections::HashSet;

pub fn solve(input: &str, bathroom_size: Point, is_p2: bool) -> (usize, Option<i32>) {
    let bots = parse(input);
    let mut time = None;
    let mut p1 = 0;

    let mut bot_copy = bots.clone();
    // tree must be within the LCM of size.x and size.y
    for t in 0..(bathroom_size.x * bathroom_size.y).max(100) {
        update(&mut bot_copy, bathroom_size);
        if is_p2 && is_tree(&bot_copy) {
            time = Some(t + 1); // since first timestep at t = 0

            if t > 100 {
                break;
            }
        }

        if t == 99 {
            p1 = get_safety_factor(&bot_copy, bathroom_size);
            if !is_p2 {
                break;
            }
        }
    }

    // used to print tree
    // if let Some(t) = time {
    //     let mut bots = bots.to_vec();
    //     for _ in 0..t {
    //         update(&mut bots, bathroom_size);
    //     }
    //     _print_grid(&bots, bathroom_size);
    // }

    (p1, time)
}

fn update(bots: &mut Vec<Bot>, bathroom_size: Point) {
    for bot in bots {
        bot.pos = bot.pos.add(bot.velocity);

        // checks for in bounds, assumes speed for each axis is not greater than bathroom size
        if bot.pos.x >= bathroom_size.x {
            bot.pos.x = bot.pos.x % bathroom_size.x;
        } else if bot.pos.x < 0 {
            bot.pos.x = bathroom_size.x + bot.pos.x;
        }

        if bot.pos.y >= bathroom_size.y {
            bot.pos.y = bot.pos.y % bathroom_size.y;
        } else if bot.pos.y < 0 {
            bot.pos.y = bathroom_size.y + bot.pos.y;
        }
    }
}

fn get_safety_factor(bots: &[Bot], size: Point) -> usize {
    let mut quad = [0; 4];

    for bot in bots {
        assert!(bot.pos.x >= 0 && bot.pos.y >= 0);
        assert!(bot.pos.x < size.x && bot.pos.y < size.y);
        if let Some(q) = match (bot.pos.x, bot.pos.y) {
            (x, y) if (x >= (size.x + 1) / 2) && (y >= (size.y + 1) / 2) => Some(0),
            (x, y) if (x >= (size.x + 1) / 2) && (y < size.y / 2) => Some(1),
            (x, y) if (x < size.x / 2) && y >= ((size.y + 1) / 2) => Some(2),
            (x, y) if (x < size.x / 2) && (y < size.y / 2) => Some(3),
            _ => None,
        } {
            quad[q] += 1;
        }
    }

    // get product of all quadrants
    let mut val = 1;
    for q in quad {
        val *= q;
    }

    val
}

fn is_tree(bots: &[Bot]) -> bool {
    // checks that no bots overlap
    let mut occupied = HashSet::new();
    for bot in bots {
        if !occupied.insert(bot.pos) {
            return false;
        }
    }

    true
}

fn parse(input: &str) -> Vec<Bot> {
    let mut bots = vec![];

    for line in input.lines() {
        let mut nums = line
            .split(|ch: char| !ch.is_numeric() && ch != '-')
            .filter(|slice| !slice.is_empty())
            .map(|slice| slice.parse::<i32>().expect("no num"));

        bots.push(Bot::new(
            Point::new(nums.next().unwrap(), nums.next().unwrap()),
            Point::new(nums.next().unwrap(), nums.next().unwrap()),
        ));
    }

    bots
}

// for debug
fn _print_grid(bots: &[Bot], size: Point) {
    for y in 0..size.y {
        println!();
        for x in 0..size.x {
            let mut count = 0;
            for bot in bots {
                if bot.pos.x == x && bot.pos.y == y {
                    count += 1;
                }
            }
            if count > 0 {
                print!("{count}");
            } else {
                print!(".");
            }
        }
    }
    println!();
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    pos: Point,
    velocity: Point,
}

impl Bot {
    fn new(pos: Point, velocity: Point) -> Self {
        Bot { pos, velocity }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&self, other: Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
