use day22::solve;

fn main() {
    let input = std::fs::read_to_string("example2.txt").unwrap();
    let ans = solve(&input);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// /*
// Answer for example is: 37327623, 0
// Answer is: 17163502021, 1938
// */
