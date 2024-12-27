pub fn solve(input: &str) -> (usize, usize) {
    let (keys, locks) = parse(input);
    (part1(&keys, &locks), part2())
}

fn part1(keys: &[[usize; 5]], locks: &[[usize; 5]]) -> usize {
    let mut fits = 0;
    for key in keys {
        'combo: for lock in locks {
            for pin in 0..=4 {
                if key[pin] + lock[pin] > 5 {
                    continue 'combo;
                }
            }

            // passed all 5 pins
            fits += 1;
        }
    }

    fits
}

fn part2() -> usize {
    0
}

fn parse(input: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut keys = vec![];
    let mut locks = vec![];

    for part in input.split("\n\n") {
        // checks if the part is a key or lock
        if part.chars().next().unwrap() == '.' {
            let mut pins = [0; 5];

            for (num, line) in part.lines().rev().enumerate() {
                let mut chars = line.chars();

                for pin_num in 0..=4 {
                    if chars.next().unwrap() == '#' {
                        pins[pin_num] = num;
                    }
                }
            }

            keys.push(pins);
        } else {
            let mut pins = [0; 5];

            for (num, line) in part.lines().enumerate() {
                let mut chars = line.chars();

                for pin_num in 0..=4 {
                    if chars.next().unwrap() == '#' {
                        pins[pin_num] = num;
                    }
                }
            }

            locks.push(pins);
        }
    }

    (keys, locks)
}

/*

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

*/
