use day06::solve;

// ngl i have no idea what number is best
const THREAD_COUNT: usize = 16;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input, THREAD_COUNT);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input, THREAD_COUNT);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// 41, 6
// 5516, 2008
