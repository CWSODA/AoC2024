use std::collections::HashSet;

pub fn solve(input: &str) -> (usize, usize) {
    let (grid, trailheads) = parse(input);
    let mut score = 0;
    let mut rating = 0;
    for th in trailheads {
        // use hashset to count peaks(9) reached by each trailhead to avoid duplicates
        let mut peaks = HashSet::new();
        rating += find_up(&grid, 0, th, &mut peaks);
        score += peaks.len();
    }

    (score, rating)
}

// returns the rating
fn find_up(grid: &[Vec<u32>], height: u32, point: Point, peaks: &mut HashSet<Point>) -> usize {
    let mut rating = 0;
    for next_point in get_cross(point) {
        if let Some(next_height) = get_grid(grid, next_point) {
            if next_height == height + 1 {
                // reached a peak(9)
                if next_height == 9 {
                    peaks.insert(next_point);
                    rating += 1; // increase per path
                    continue;
                }
                rating += find_up(grid, next_height, next_point, peaks);
            }
        }
    }

    rating
}

// returns number at point, returns None if out of bounds
fn get_grid(grid: &[Vec<u32>], point: Point) -> Option<u32> {
    if point.x < 0 || point.y < 0 {
        return None;
    }
    Some(*grid.get(point.y as usize)?.get(point.x as usize)?)
}

fn get_cross(point: Point) -> Vec<Point> {
    // check the cross, possible paths
    // . x .
    // x o x
    // . x .
    let mut out = vec![];
    out.push(Point::new(point.x + 1, point.y));
    out.push(Point::new(point.x - 1, point.y));
    out.push(Point::new(point.x, point.y + 1));
    out.push(Point::new(point.x, point.y - 1));

    out
}

// for debug
fn _print_path(grid: &[Vec<u32>], path: &HashSet<Point>) {
    for (y, row) in grid.iter().enumerate() {
        println!();
        for (x, height) in row.iter().enumerate() {
            if path.contains(&Point::new(x as i32, y as i32)) {
                print!("{height}");
            } else {
                print!(".");
            }
        }
    }
    println!();
}

// returns grid and trailheads
fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Point>) {
    let mut grid = vec![];
    let mut trailheads = vec![];

    for (y, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let digit = c.to_digit(10).expect("Numbers only!");
                    if digit == 0 {
                        trailheads.push(Point::new(x as i32, y as i32));
                    }
                    digit
                })
                .collect(),
        );
    }

    (grid, trailheads)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
