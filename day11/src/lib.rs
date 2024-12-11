use hashbrown::HashMap;

pub fn solve(input: &str, blinks: usize) -> usize {
    // keep track of previously seen
    // (stone, blinks_left), count after blinks
    let mut seen: HashMap<(u64, usize), usize> = HashMap::new();

    input
        .split_whitespace()
        .map(|num| stoning(num.parse().unwrap(), blinks, &mut seen))
        .sum()
}

fn stoning(stone: u64, blinks: usize, seen: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    } else if let Some(cached) = seen.get(&(stone, blinks)) {
        return *cached;
    }

    let val = if stone == 0 {
        stoning(1, blinks - 1, seen)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let mask = 10u64.pow(digits / 2);
            stoning(stone / mask, blinks - 1, seen) + stoning(stone % mask, blinks - 1, seen)
        } else {
            stoning(stone * 2024, blinks - 1, seen)
        }
    };

    seen.insert((stone, blinks), val);
    val
}
