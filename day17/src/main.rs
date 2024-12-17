use day17::solve;

fn main() {
    let input = std::fs::read_to_string("example_p2.txt").unwrap();
    let (output, min_reg_a) = solve(&input);
    println!("Example: {output}\nMin RegA is {min_reg_a}");

    println!();

    let input = std::fs::read_to_string("input.txt").unwrap();
    let (output, min_reg_a) = solve(&input);
    println!("Real Input: {output}\nMin RegA is {min_reg_a}");
}

/*
Example: 0,3,5,4,3,0
Min RegA is 117440

Real Input: 1,4,6,1,6,4,3,0,3
Min RegA is 265061364597659
*/
