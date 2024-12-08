use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (usize, usize) {
    let (points, grid_size) = parse(input);
    (part1(&points, grid_size), part2(&points, grid_size))
}

fn part1(points: &HashMap<char, Vec<Point>>, grid_size: Point) -> usize {
    let mut nodes: HashSet<Point> = HashSet::new();

    // loop through all unique pairs for each char
    for freq in points.values() {
        for i in 0..(freq.len() - 1) {
            let p1 = freq[i];
            for p2 in freq.iter().skip(i + 1) {
                let diff = p1.sub(*p2);
                if let Some(new_p) = p1.add(diff).in_grid(grid_size) {
                    nodes.insert(new_p);
                }
                if let Some(new_p) = p2.sub(diff).in_grid(grid_size) {
                    nodes.insert(new_p);
                }
            }
        }
    }

    nodes.len()
}

fn part2(points: &HashMap<char, Vec<Point>>, grid_size: Point) -> usize {
    let mut nodes: HashSet<Point> = HashSet::new();

    // loop through all unique pairs for each char
    for freq in points.values() {
        for i in 0..(freq.len() - 1) {
            let p1 = freq[i];
            for p2 in freq.iter().skip(i + 1) {
                let diff = p1.sub(*p2);

                let mut count = 1;
                while let Some(new_p) = p1.add(diff.mul(count)).in_grid(grid_size) {
                    nodes.insert(new_p);
                    count += 1;
                }

                let mut count = 1;
                while let Some(new_p) = p2.sub(diff.mul(count)).in_grid(grid_size) {
                    nodes.insert(new_p);
                    count += 1;
                }
            }
        }

        // each point is also a node
        for p in freq {
            nodes.insert(*p);
        }
    }

    nodes.len()
}

fn parse(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut points = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate().filter(|c| c.1 != '.') {
            points
                .entry(ch)
                .and_modify(|v: &mut Vec<_>| v.push(Point::new(x as i32, y as i32)))
                .or_insert(vec![Point::new(x as i32, y as i32)]);
        }
    }

    (
        points,
        Point::new(
            input.lines().next().unwrap().chars().count() as i32,
            input.lines().count() as i32,
        ),
    )
}

// used to debug
fn _print_grid(nodes: &HashSet<Point>, grid_size: Point) {
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            print!(
                "{}",
                if nodes.contains(&Point::new(x, y)) {
                    '#'
                } else {
                    '.'
                }
            )
        }
        println!();
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn sub(&self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    fn add(&self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    fn mul(&self, b: i32) -> Point {
        Point::new(self.x * b, self.y * b)
    }

    fn in_grid(&self, grid_size: Point) -> Option<Point> {
        if self.x < grid_size.x && self.y < grid_size.y && self.y >= 0 && self.x >= 0 {
            Some(*self)
        } else {
            None
        }
    }
}

// 14, 34
// 261, 898
