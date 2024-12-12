use hashbrown::{HashMap, HashSet};

pub fn solve(input: &str) -> (usize, usize) {
    let crops = parse(input);

    let mut val1 = 0;
    let mut val2 = 0;

    for (_plant, points) in crops {
        let mut regions: Vec<HashSet<Point>> = vec![];

        // xxx.xxx
        // ..xxx..
        let mut points: Vec<Point> = points.clone().into_iter().collect();
        points.sort();

        // sort the points into their own regions
        while points.len() != 0 {
            let p = points.pop().unwrap();
            let mut region = HashSet::from([p]);
            walk(p, &mut points, &mut region);

            regions.push(region);
        }

        for region in &regions {
            let mut perim = 0;
            let mut corners = 0;
            for point in region {
                perim += 4 - point.adj_plots(region);
                corners += point.corners(region);
            }

            // area = region.len() since each point is 1 area
            val1 += region.len() * perim;

            println!("{_plant} has {corners} corners");
            val2 += region.len() * corners;
        }
    }

    (val1, val2)
}

fn walk(p: Point, points: &mut Vec<Point>, set: &mut HashSet<Point>) {
    // walks through all neighbors
    for adj_p in p.adj_points() {
        if let Ok(index) = points.binary_search(&adj_p) {
            set.insert(points.remove(index));
            walk(adj_p, points, set);
        }
    }
}

fn parse(input: &str) -> HashMap<char, HashSet<Point>> {
    let mut crops = HashMap::new();

    for (y, row) in input.lines().enumerate() {
        for (x, plant) in row.chars().enumerate() {
            let pos = Point::new(x as i32, y as i32);
            crops
                .entry(plant)
                .and_modify(|place: &mut HashSet<Point>| {
                    place.insert(pos);
                })
                .or_insert(HashSet::from([pos]));
        }
    }

    crops
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn adj_points(&self) -> Vec<Point> {
        vec![
            self.add(-1, 0),
            self.add(1, 0),
            self.add(0, -1),
            self.add(0, 1),
        ]
    }

    fn adj_plots(&self, set: &HashSet<Point>) -> usize {
        let mut val = 0;

        if set.contains(&self.add(-1, 0)) {
            val += 1;
        }
        if set.contains(&self.add(1, 0)) {
            val += 1;
        }
        if set.contains(&self.add(0, -1)) {
            val += 1;
        }
        if set.contains(&self.add(0, 1)) {
            val += 1;
        }

        val
    }

    fn corners(&self, region: &HashSet<Point>) -> usize {
        let mut corners = 0;
        // match all relevant points in the order:
        // 012
        // 3x4
        // 567
        //
        // convert to possible corners (4 total)
        // bc
        // ax
        let around = [
            region.contains(&self.add(-1, -1)),
            region.contains(&self.add(0, -1)),
            region.contains(&self.add(1, -1)),
            region.contains(&self.add(-1, 0)),
            region.contains(&self.add(1, 0)),
            region.contains(&self.add(-1, 1)),
            region.contains(&self.add(0, 1)),
            region.contains(&self.add(1, 1)),
        ];

        let possibles = [
            [around[3], around[0], around[1]],
            [around[1], around[2], around[4]],
            [around[4], around[7], around[6]],
            [around[6], around[5], around[3]],
        ];

        for possible in possibles {
            match possible {
                // ..
                // .x
                // or
                // .x
                // xx
                // or
                // x.
                // .x
                [true, false, true] | [false, false, false] | [false, true, false] => corners += 1,
                _ => {}
            }
        }

        corners
    }
}

// for debug
fn _print_corners(around: &[bool], corners: usize) {
    println!("Region with {corners} corner(s)");
    println!("{}{}{}", _p(around[0]), _p(around[1]), _p(around[2]));
    println!("{}x{}", _p(around[3]), _p(around[4]));
    println!("{}{}{}", _p(around[5]), _p(around[6]), _p(around[7]));
}

fn _p(input: bool) -> char {
    if input {
        'x'
    } else {
        '.'
    }
}
