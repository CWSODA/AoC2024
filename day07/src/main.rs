use day07::solve;

const THREAD_COUNT: usize = 16;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input, THREAD_COUNT);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input, THREAD_COUNT);
    println!("Answer is: {}, {}", ans.0, ans.1);
}
