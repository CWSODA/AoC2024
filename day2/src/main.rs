fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut val = 0;

    'outer: for line in input.lines() {
        let list: Vec<i32> = line
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        if !is_safe(&list) {
            continue 'outer;
        }

        val += 1;
    }

    return val;
}

fn part2(input: &str) -> i32 {
    let mut val = 0;

    'perline: for line in input.lines() {
        let list: Vec<i32> = line
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        if is_safe(&list) {
            val += 1;
            continue 'perline;
        }

        // needs removing to potentially be safe
        for index in 0..(list.len()) {
            let mut temp = list.clone();
            temp.remove(index);

            if is_safe(&temp) {
                val += 1;
                continue 'perline;
            }

            // otherwise continue the loop
        }
    }

    return val;
}

fn is_safe(temp: &[i32]) -> bool {
    let mut list_sort = temp.to_vec();
    list_sort.sort();
    let mut list_rev = list_sort.clone();
    list_rev.reverse();

    if temp != list_sort && temp != list_rev {
        return false;
    }

    let mut last = temp.first().unwrap();

    for num in temp.iter().skip(1) {
        if num == last || (last - num).abs() > 3 {
            return false;
        }

        last = num;
    }

    return true;
}
