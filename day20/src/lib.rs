pub fn solve(input: &str) -> (usize, usize) {
    let (grid, start, end) = parse(input);
    let costs = walk(&grid, start, end);
    let mut p1 = 0;
    let mut p2 = 0;

    let cheats = cheat_finder(&costs, 20);

    for cheat in cheats {
        // time saved is time between start and end, minus the duration of cheat
        // to avoid underflow with usize, add the duration to other side
        if cheat.end_cost - cheat.start_cost >= 100 + cheat.duration {
            if cheat.duration == 2 {
                p1 += 1;
            }
            p2 += 1;
        }
    }

    (p1, p2)
}

// finds all possible cheats within the max duration
fn cheat_finder(path: &[Vec2], max_duration: usize) -> Vec<Cheat> {
    let mut cheats = vec![];

    for (start_cost, cheat_start) in path.iter().enumerate() {
        for (end_cost, cheat_end) in path.iter().enumerate().skip(start_cost + 2) {
            let duration = cheat_start.dist_to(cheat_end);
            if duration <= max_duration {
                cheats.push(Cheat {
                    start_cost,
                    end_cost,
                    duration,
                });
            }
        }
    }

    cheats
}

// since single path, returns Vec of each position, including start and end
// the cost for each tile is thus simply the index
fn walk(grid: &[Vec<Tile>], start: Vec2, end: Vec2) -> Vec<Vec2> {
    let mut path = vec![start];

    let mut pos = start;
    let mut from = start;

    while pos != end {
        // finds next tile from neighbors, excludes tile it came from
        for new_pos in pos.around() {
            if grid[new_pos.y][new_pos.x] != Tile::Wall && new_pos != from {
                from = pos;
                pos = new_pos;
                path.push(new_pos);
                break;
            }
        }
    }

    path
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec2, Vec2) {
    let mut start = None;
    let mut end = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    match ch {
                        '#' => return Tile::Wall,
                        'S' => start = Some(Vec2::new(x, y)),
                        'E' => end = Some(Vec2::new(x, y)),
                        '.' => { /* Do Nothing */ }
                        _ => panic!("Invalid Character!"),
                    };
                    Tile::Path
                })
                .collect()
        })
        .collect();

    (grid, start.expect("No Start!"), end.expect("No End!"))
}

#[derive(PartialEq)]
enum Tile {
    Path,
    Wall,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }

    fn around(&self) -> [Self; 4] {
        [
            Vec2::new(self.x + 1, self.y),
            Vec2::new(self.x - 1, self.y),
            Vec2::new(self.x, self.y + 1),
            Vec2::new(self.x, self.y - 1),
        ]
    }

    fn dist_to(&self, other: &Vec2) -> usize {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

struct Cheat {
    start_cost: usize,
    end_cost: usize,
    duration: usize,
}
