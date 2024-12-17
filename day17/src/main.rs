use day9::solve;

fn main() {
    let input = std::fs::read_to_string("example2.txt").unwrap();
    print!("Example: ");
    println!("RegA is {}", solve(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    print!("\nReal Input: ");
    println!("RegA is {}", solve(&input));
}
