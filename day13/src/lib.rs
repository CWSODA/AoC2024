use itertools::Itertools;

pub fn solve(input: &str) -> (i64, i64) {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for machine in parse(input) {
        if let Some((a, b)) = machine.solve(false) {
            sum1 += a * 3 + b;
        }
        if let Some((a, b)) = machine.solve(true) {
            sum2 += a * 3 + b;
        }
    }

    (sum1, sum2)
}

fn parse(input: &str) -> Vec<Machine> {
    let mut output = vec![];

    for mach in &input.lines().chunks(4) {
        let mut machine_info = vec![];
        for line in mach.take(3) {
            let mut iter = line
                .split(|c: char| !c.is_numeric())
                .filter(|x| !x.is_empty())
                .map(|ch| ch.parse::<i64>().expect("no_num"));

            machine_info.push(Point::new(
                iter.next().expect("cant get x"),
                iter.next().expect("cant get y"),
            ));
        }

        output.push(Machine::new(
            machine_info[0],
            machine_info[1],
            machine_info[2],
        ));
    }

    output
}

#[derive(Clone, Copy, Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

impl Machine {
    fn new(a: Point, b: Point, prize: Point) -> Self {
        Machine { a, b, prize }
    }

    // solves the linear system of equations
    // A(ax,ay) + B(bx,by) = px,py
    fn solve(&self, is_p2: bool) -> Option<(i64, i64)> {
        let prize = if is_p2 {
            Point::new(self.prize.x + 10000000000000, self.prize.y + 10000000000000)
        } else {
            self.prize
        };

        let det = self.a.x * self.b.y - self.b.x * self.a.y;
        if det == 0 {
            // no singular solution
            // this line is never reached lol
            // don't know why they didn't include some linearly dependent machines
            // but solution would be to just add the smaller of (a) or (b) weighted by cost
            println!("no sol");
            return None;
        }

        // inverse of the matrix without the scalar (1/det)
        let x_num = prize.x * self.b.y - prize.y * self.b.x;
        let y_num = -prize.x * self.a.y + prize.y * self.a.x;

        // check if is integer
        if x_num % det == 0 && y_num % det == 0 {
            return Some((x_num / det, y_num / det));
        }

        None
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}
