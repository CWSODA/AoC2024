use day22::solve;

fn main() {
    let input = include_str!("../input.txt");
    let ans = solve(&input);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

// Answer is: 17163502021, 1938
