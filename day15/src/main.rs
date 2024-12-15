use day15::solve;

fn main() {
    let input = std::fs::read_to_string("example_big.txt").unwrap();
    let ans = solve(&input);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// Answer for example is: 10092, 9021
// Answer is: 1463512, 1486520
