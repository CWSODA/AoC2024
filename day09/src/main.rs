use day09::solve;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// 1928, 2858
// 6291146824486, 6307279963620
