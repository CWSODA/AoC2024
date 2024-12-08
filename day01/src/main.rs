use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("example.txt").expect("Can't open file");
    let lists = parse(&input);
    println!("Example value for part 1:\n{}", part1(&lists));
    println!("Example value for part 2:\n{}", part2(&lists));

    let input = std::fs::read_to_string("input.txt").expect("Can't open file");
    let lists = parse(&input);
    println!("Value for part 1:\n{}", part1(&lists));
    println!("Value for part 2:\n{}", part2(&lists));
}

fn part1(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|n| (n.0 - n.1).abs())
        .sum()
}

fn part2(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut sum = 0;

    for num in lists.0.iter() {
        for other in lists.1.iter() {
            match num.cmp(other) {
                Ordering::Equal => sum += num,
                Ordering::Less => break,
                _ => {}
            }
        }
    }

    sum
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first: Vec<i32> = vec![];
    let mut second: Vec<i32> = vec![];

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        first.push(
            numbers
                .next()
                .expect("Need 2 lists")
                .parse()
                .expect("List should be num"),
        );
        second.push(
            numbers
                .next()
                .expect("Need 2 lists")
                .parse()
                .expect("List should be num"),
        );

        assert!(numbers.next().is_none(), "Invalid number of lists")
    }

    first.sort();
    second.sort();

    (first, second)
}

// 11,31
// 3508942, 26593248
