use std::ops::Index;

use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_y};

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));

    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Value for part 1 is:\n{}", part1(&input));
    println!("Value for part 2 is:\n{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut val = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        val += line.matches("XMAS").count();
        val += line.matches("SAMX").count();

        grid.push(line.chars().collect());
    }

    let bars = straight_y(&grid);

    for line in bars {
        let s: String = line.iter().map(|c| **c).collect();
        val += s.matches("XMAS").count();
        val += s.matches("SAMX").count();
    }

    // 1 2 3 4
    // 2 x x x
    // 3 x x x
    // 4 x x x
    // 5 x x x
    let diag1 = diagonal_pos_pos(&grid);
    let diag2 = diagonal_pos_neg(&grid);

    for line in diag1 {
        let s: String = line.iter().map(|c| **c).collect();
        val += s.matches("XMAS").count();
        val += s.matches("SAMX").count();
    }
    for line in diag2 {
        let s: String = line.iter().map(|c| **c).collect();
        val += s.matches("XMAS").count();
        val += s.matches("SAMX").count();
    }

    return val;
}

//println!("X-MAS at {col}, {row}");
fn part2(input: &str) -> usize {
    let mut val = 0;

    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    // M.M     S.M
    // .A.     .A.
    // S.S     S.M

    // always a buffer for A
    // skip the first line
    let col_size = grid.len();
    let row_size = grid[0].len();

    for (mut col, line) in grid.iter().skip(1).enumerate() {
        col += 1; // since skipped first
        if col >= col_size - 1 {
            // skip last
            break;
        }
        for (mut row, c) in line.iter().skip(1).enumerate() {
            row += 1; // since skipped first
            if row >= row_size - 1 {
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

    return val;
}
