use day14::{solve, Point};

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input, Point::new(11, 7), false);
    println!("Answer for example is: {}", ans.0);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input, Point::new(101, 103), true);
    println!("Answer is: {}, {:?}", ans.0, ans.1);
}
