use hashbrown::HashMap;

pub fn _solve(input: &str, blinks: usize) -> usize {
    // keep track of previously seen
    // (stone, blinks_left), count after blinks
    let mut seen: HashMap<(u64, usize), usize> = HashMap::new();

    input
        .split_whitespace()
        .map(|num| _stoning(num.parse().unwrap(), blinks, &mut seen))
        .sum()
}

fn _stoning(stone: u64, blinks: usize, seen: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    } else if let Some(cached) = seen.get(&(stone, blinks)) {
        return *cached;
    }

    let val = if stone == 0 {
        _stoning(1, blinks - 1, seen)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let mask = 10u64.pow(digits / 2);
            _stoning(stone / mask, blinks - 1, seen) + _stoning(stone % mask, blinks - 1, seen)
        } else {
            _stoning(stone * 2024, blinks - 1, seen)
        }
    };

    seen.insert((stone, blinks), val);
    val
}

pub fn alt_solve(input: &str, blinks: (usize, usize)) -> (usize, usize) {
    assert!(blinks.1 > blinks.0);
    let mut steins: HashMap<u64, usize> = input
        .split(' ')
        .map(|num| (num.parse().unwrap(), 1))
        .collect();

    let mut p1 = 0;
    for b in 0..(blinks.1) {
        if b == blinks.0 {
            p1 = steins.values().sum();
        }
        steins = steins_gate(&steins);
    }

    (p1, steins.values().sum())
}

// hashmap of stone number, count
fn steins_gate(old_steins: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_steins: HashMap<u64, usize> = HashMap::with_capacity(old_steins.len());

    for (stein, count) in old_steins {
        match stein {
            0 => *new_steins.entry(1).or_default() += *count,
            _ => {
                let digits = stein.ilog10() + 1;
                if digits % 2 == 0 {
                    let mask = 10u64.pow(digits / 2);
                    *new_steins.entry(stein / mask).or_default() += *count;
                    *new_steins.entry(stein % mask).or_default() += *count;
                } else {
                    *new_steins.entry(stein * 2024).or_default() += *count;
                }
            }
        }
    }

    new_steins
}
