use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> (u64, u64) {
    let mut sum = 0;
    // stores the 4 changes, and the highest price
    let mut futures: HashMap<[i8; 4], u64> = HashMap::new();

    for base in input
        .lines()
        .map(|line| line.parse::<u64>().expect("Not a number!"))
    {
        let mut secret = base;
        let mut current_future: HashMap<[i8; 4], u64> = HashMap::new();

        let mut sequence = VecDeque::from([(secret % 10) as i8]);

        for _ in 0..2000 {
            secret = get_next(secret);

            let price = secret % 10;

            sequence.push_back(price as i8);
            if sequence.len() >= 6 {
                sequence.pop_front();
            }

            // make sure there are 5 prices in sequence, hence 4 changes
            if sequence.len() != 5 {
                continue;
            }

            current_future
                .entry(get_changes(&sequence))
                .or_insert(price);
        }

        // merge with all futures
        for (key, price) in current_future {
            futures
                .entry(key)
                .and_modify(|prev_price| *prev_price += price)
                .or_insert(price as u64);
        }
        sum += secret;
    }

    (
        sum,
        *futures.values().max().expect("Failed to get best price!"),
    )
}

fn get_changes(sequence: &VecDeque<i8>) -> [i8; 4] {
    assert!(sequence.len() == 5);
    [
        sequence[1] - sequence[0],
        sequence[2] - sequence[1],
        sequence[3] - sequence[2],
        sequence[4] - sequence[3],
    ]
}

fn get_next(mut secret: u64) -> u64 {
    // step 1: multi by 64, mix, prune
    secret = prune(mix(secret << 6, secret));

    // step 2: div by 32, mix, prune
    secret = prune(mix(secret >> 5, secret));

    // step 3: multi by 2048, mix, prune
    secret = prune(mix(secret << 11, secret));

    secret
}

fn mix(secret: u64, other: u64) -> u64 {
    secret ^ other
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve(include_str!("../example.txt"));
        assert_eq!(result.0, 37327623);
    }

    #[test]
    fn part2_example() {
        let result = solve(include_str!("../example2.txt"));
        assert_eq!(result.1, 23);
    }
}
