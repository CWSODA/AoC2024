use day01::solve;

fn main() {
    let input = std::fs::read_to_string("example.txt").expect("Can't open file");
    let ans = solve(&input);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").expect("Can't open file");
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// 11,31
// 3508942, 26593248
