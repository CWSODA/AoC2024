use std::ops::Index;

use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_y};

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let grid = parse(&input);
    println!("Value for part 1 is:\n{}", part1(&grid));
    println!("Value for part 2 is:\n{}", part2(&grid));

    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse(&input);
    println!("Value for part 1 is:\n{}", part1(&grid));
    println!("Value for part 2 is:\n{}", part2(&grid));
}

fn part1(grid: &[Vec<char>]) -> usize {
    let mut val = 0;

    for line in grid {
        let mut window = String::new();

        for c in line {
            window.push(*c);
            if window.len() > 4 {
                window.remove(0);
            }

            if window == "XMAS" || window == "SAMX" {
                val += 1;
            }
        }
    }

    let bars = straight_y(&grid);
    let diag1 = diagonal_pos_pos(&grid);
    let diag2 = diagonal_pos_neg(&grid);

    for line in bars {
        val += match_xmas(&line);
    }
    for line in diag1 {
        val += match_xmas(&line);
    }
    for line in diag2 {
        val += match_xmas(&line);
    }

    val
}

fn match_xmas(input: &[&char]) -> usize {
    let mut window = String::new();
    let mut val = 0;

    for c in input {
        window.push(**c);
        if window.len() > 4 {
            window.remove(0);
        }

        if window == "XMAS" || window == "SAMX" {
            val += 1;
        }
    }

    val
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut val = 0;

    // M.M     S.M
    // .A.     .A.
    // S.S     S.M

    // always letters around A, skip the first line
    for (col, line) in grid.iter().enumerate().skip(1) {
        if col >= grid.len() - 1 {
            // skip last
            break;
        }
        for (row, c) in line.iter().enumerate().skip(1) {
            if row >= grid[0].len() - 1 {
                // skip last
                break;
            }

            if *c != 'A' {
                continue;
            }

            // matches:
            // A B
            // C D
            match (
                grid[col - 1].index(row - 1),
                grid[col - 1].index(row + 1),
                grid[col + 1].index(row - 1),
                grid[col + 1].index(row + 1),
            ) {
                ('M', 'M', 'S', 'S')
                | ('M', 'S', 'M', 'S')
                | ('S', 'M', 'S', 'M')
                | ('S', 'S', 'M', 'M') => val += 1,
                _ => { /*  do nothing */ }
            }
        }
    }

    val
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// 18, 9
// 2578, 1972
