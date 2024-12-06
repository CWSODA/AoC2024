fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    println!("Example value for part 1:\n{}", part1(&input));

    let input = std::fs::read_to_string("example2.txt").unwrap();
    println!("Example value for part 2:\n{}", part2(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Value for part 1:\n{}", part1(&input));
    println!("Value for part 2:\n{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut val = 0;

    // match with mul(num,num)
    for s in input.split("mul(").skip(1) {
        let slice = match s.find(')') {
            Some(index) => s.split_at(index).0,
            None => continue, // if no closing bracket then invalid
        };

        // ensures slice only contains one comma and numbers
        let mut comma_count = 0;
        if !slice.chars().all(|c| {
            if c == ',' {
                comma_count += 1;
                true
            } else {
                c.is_numeric()
            }
        }) || comma_count != 1
        {
            continue;
        }

        // thus there should only be numbers left
        let nums: Vec<&str> = slice.split(',').collect();
        if nums.len() != 2 {
            continue; // not enough numbers, invalid
        }

        val += nums[0].parse::<i32>().unwrap() * nums[1].parse::<i32>().unwrap();
    }

    val
}

fn part2(input: &str) -> i32 {
    // takes everything in between do()
    // note: do is enabled by default so first part is still valid
    input
        .split("do()")
        .map(|part| part1(part.split("don't()").next().unwrap()))
        .sum()

    // xmul(2,4)&mul[3,7]!^  don't()  _mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    // xmul(2,4)&mul[3,7]!^  don't()  _mul(5,5)+mul(32,64](mul(11,8)un
    // ?mul(8,5))
    // only take first slice before don't
}

// 161, 48
// 173419328, 90669332
