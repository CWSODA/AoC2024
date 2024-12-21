use day20::solve;

fn main() {
    // output for example is redundant, maybe I should start writing test modules...

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// Answer is: 1530, 1033983
