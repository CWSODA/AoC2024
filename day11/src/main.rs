use day11::alt_solve;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = alt_solve(&input, (25, 75));
    // println!("Answer is: {}, {}", solve(&input, 25), solve(&input, 75));
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// 233050, 276661131175807
