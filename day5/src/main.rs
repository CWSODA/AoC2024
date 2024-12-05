use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let mut rules: HashSet<(i32, i32)> = HashSet::new();
    let mut updates = vec![];
    let mut ans;

    let input = std::fs::read_to_string("example.txt").unwrap();
    parse(&input, &mut updates, &mut rules);
    ans = solve(&updates, &rules);
    println!("Answer for example is: {}, {}", ans.0, ans.1);

    let input = std::fs::read_to_string("input.txt").unwrap();
    parse(&input, &mut updates, &mut rules);
    ans = solve(&updates, &rules);
    println!("Answer is: {}, {}", ans.0, ans.1);
}

fn solve(updates: &[Vec<i32>], rules: &HashSet<(i32, i32)>) -> (i32, i32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for update in updates {
        if is_valid(&update, &rules) {
            p1 += update[(update.len() - 1) / 2];
        } else {
            p2 += part2(update, rules);
        }
    }

    return (p1, p2);
}

fn part2(invalid_update: &[i32], rules: &HashSet<(i32, i32)>) -> i32 {
    let mut val = 0;
    let mut sorted = invalid_update.to_vec();

    sorted.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            return Ordering::Less;
        }

        Ordering::Greater
    });

    val += sorted[(sorted.len() - 1) / 2];

    return val;
}

// checks if a given update is valid
fn is_valid(update: &Vec<i32>, rules: &HashSet<(i32, i32)>) -> bool {
    for (i1, p1) in update.iter().enumerate() {
        // takes index of all rules that involve the number as its first arg
        for (_, b) in rules.iter().filter(|r| r.0 == *p1) {
            if let Some(i2) = update.iter().position(|n| *n == *b) {
                if i2 < i1 {
                    return false;
                }
            }
        }
    }

    true
}

// parses input into updates and rules
fn parse(input: &str, updates: &mut Vec<Vec<i32>>, rules: &mut HashSet<(i32, i32)>) {
    rules.clear();
    updates.clear();

    for line in input.lines() {
        // takes note of all the rules
        if line.contains('|') {
            let mut i = line.split('|');
            rules.insert((
                i.next().unwrap().parse().unwrap(),
                i.next().unwrap().parse().unwrap(),
            ));
            if i.next().is_some() {
                panic!()
            }
        } else if line.contains(',') {
            // get the numbers in a line
            updates.push(line.split(',').map(|c| c.parse::<i32>().unwrap()).collect());
        }
    }
}
