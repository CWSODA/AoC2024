use hashbrown::HashMap;

pub fn solve(input: &str) -> (usize, usize) {
    let (towels, designs) = parse(input);

    let mut possible_designs = 0;
    let mut combinations = 0;

    let mut memo = HashMap::new();
    for design in designs.lines() {
        let n = check(&towels, design, &mut memo);
        if n > 0 {
            possible_designs += 1;
            combinations += n;
        }
    }

    (possible_designs, combinations)
}

fn check<'a>(towels: &[&str], design: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(n) = memo.get(design) {
        return *n;
    }

    let n = towels
        .iter()
        .filter(|t| design.starts_with(*t))
        .map(|t| check(towels, &design[t.len()..], memo))
        .sum();

    memo.insert(design, n);
    n
}

fn parse(input: &str) -> (Vec<&str>, &str) {
    let (a, designs) = input.split_once("\n\n").expect("Invalid format");

    let towels = a.split(", ").collect();

    (towels, designs)
}
