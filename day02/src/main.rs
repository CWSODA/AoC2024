fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let reports = parse(&input);
    println!("Example value for part 1:\n{}", part1(&reports));
    println!("Example value for part 2:\n{}", part2(&reports));

    let input = std::fs::read_to_string("input.txt").unwrap();
    let reports = parse(&input);
    println!("Value for part 1:\n{}", part1(&reports));
    println!("Value for part 2:\n{}", part2(&reports));
}

fn part1(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

fn part2(reports: &[Vec<i32>]) -> i32 {
    let mut val = 0;

    for report in reports {
        if is_safe(report) {
            val += 1;
            continue;
        }

        // needs removing to potentially be safe
        for index in 0..(report.len()) {
            let mut temp = report.clone();
            temp.remove(index);

            if is_safe(&temp) {
                val += 1;
                break;
            }
        }
    }

    val
}

fn is_safe(temp: &[i32]) -> bool {
    // checks if all numbers are descending or ascending
    if !temp.is_sorted() && !temp.iter().rev().is_sorted() {
        return false;
    }

    let mut last = temp.first().unwrap();

    for num in temp.iter().skip(1) {
        if num == last || (last - num).abs() > 3 {
            return false;
        }

        last = num;
    }

    true
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports = vec![];
    for report in input.lines() {
        reports.push(
            report
                .split_whitespace()
                .map(|e| e.parse::<i32>().expect("must be number"))
                .collect(),
        )
    }

    reports
}

// 2,4
// 490, 536
