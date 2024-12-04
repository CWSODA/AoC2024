fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));

    let input = std::fs::read_to_string("example2.txt").unwrap();
    println!("Value for part 2 is:\n{}", part2(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut val = 0;

    // match with mul(num,num)
    for s in input.split("mul(").skip(1) {
        let slice = match s.find(')') {
            Some(index) => s.split_at(index).0,
            None => continue, // if no closing bracket then invalid
        };

        if slice.contains(|e: char| e != ',' && !e.is_numeric()) {
            continue; // if contains something other than comma or number, invalid
        }

        if slice.chars().filter(|c| *c == ',').count() != 1 {
            continue; // if not exactly one comma, invalid
        }

        // thus there should only be numbers left
        let nums = slice.split(',');
        if nums.clone().count() != 2 {
            continue; // not enough numbers, invalid
        }

        let mut product = 1;
        for num in nums {
            product *= num.parse::<i32>().unwrap();
        }

        val += product;
    }

    // println!("Value for part 1 is:\n{val}");
    return val;
}

fn part2(input: &str) -> i32 {
    let mut val = 0;

    // takes everything in between do()
    // note do is enabled by default so first part is still valid
    for parts in input.split("do()") {
        val += part1(parts.split("don't()").next().unwrap());
    }

    // xmul(2,4)&mul[3,7]!^  don't()  _mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    // xmul(2,4)&mul[3,7]!^  don't()  _mul(5,5)+mul(32,64](mul(11,8)un
    // ?mul(8,5))
    // only take first slice before don't

    return val;
}
