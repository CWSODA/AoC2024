fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut first: Vec<i32> = vec![];
    let mut second: Vec<i32> = vec![];

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        first.push(numbers.next().unwrap().parse().unwrap());
        second.push(numbers.next().unwrap().parse().unwrap());
    }

    first.sort();
    second.sort();

    let mut sum = 0;

    for pair in first.iter().zip(second.iter()) {
        sum += (pair.0 - pair.1).abs();
    }

    return sum;
}

fn part2(input: &str) -> i32 {
    let mut first: Vec<i32> = vec![];
    let mut second: Vec<i32> = vec![];

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        first.push(numbers.next().unwrap().parse().unwrap());
        second.push(numbers.next().unwrap().parse().unwrap());
    }

    first.sort();
    second.sort();

    let mut sum = 0;

    for num in first.iter() {
        let mut count = 0;
        for other in second.iter() {
            if num == other {
                count += 1;
            } else if other > num {
                break; // break when over value since both lists are sorted
            }
        }
        sum += num * count;
    }

    return sum;
}
