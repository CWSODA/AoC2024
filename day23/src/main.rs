use day23::solve;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let ans = solve(&input);
    println!("Answer for example is: {}\nPassword is: {}\n", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let ans = solve(&input);
    println!("Answer is: {}\nPassword is: {}", ans.0, ans.1);
}

// bz-cs-fx-ms-oz-po-sy-uh-uv-vw-xu-zj-zm
