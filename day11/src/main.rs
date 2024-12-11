use day11::solve;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Answer is: {}, {}", solve(&input, 25), solve(&input, 75));
}
