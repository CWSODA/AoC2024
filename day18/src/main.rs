use day18::solve;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input, 6, 12);
    println!("Answer for example is: {}, ({})", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input, 70, 1024);
    println!("Answer is: {}, ({})", ans.0, ans.1);
}

/*
Answer for example is: 22, (6,1)
Answer is: 296, (28,44)
*/
