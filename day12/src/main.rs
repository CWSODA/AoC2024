use day12::solve;

fn main() {
    let wanted = vec![false, false, true];

    if wanted[0] {
        let input = std::fs::read_to_string("example.txt").unwrap();
        let ans = solve(&input);
        println!("Answer for example is: {}, {}", ans.0, ans.1);
    }

    if wanted[1] {
        let input = std::fs::read_to_string("ex.txt").unwrap();
        let ans = solve(&input);
        println!("Answer for ex is: {}, {}", ans.0, ans.1);
    }

    if wanted[2] {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let ans = solve(&input);
        println!("Answer is: {}, {}", ans.0, ans.1);
    }
}
