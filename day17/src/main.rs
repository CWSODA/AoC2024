use day9::solve;

fn main() {
    let input = std::fs::read_to_string("example2.txt").unwrap();
    print!("Example: ");
    println!("RegA is {}", solve(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    print!("\nReal Input: ");
    println!("RegA is {}", solve(&input));
}

/*
Example: 0,3,5,4,3,0
RegA is 117440

Real Input: 1,4,6,1,6,4,3,0,3
RegA is 265061364597659
*/
