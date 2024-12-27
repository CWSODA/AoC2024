use day24::solve;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input, Some(4));
    println!("Answer is: {}, {}", ans.0, ans.1);
}
