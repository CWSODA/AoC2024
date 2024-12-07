use std::iter::repeat_n;

use itertools::Itertools;

pub fn solve(input: &str) -> (u64, u64) {
    let equations = parse(input);

    (part1(&equations), part2(&equations))
}

fn part1(equations: &[Vec<u64>]) -> u64 {
    let mut out = 0;

    for eq in equations {
        // all operator combinations
        // 0 is add, 1 is multiply
        for comb in repeat_n([OP::MULT, OP::ADD], eq.len() - 2).multi_cartesian_product() {
            // assign val to first number
            let mut val = eq[1];

            for (num, op) in eq.iter().skip(2).zip_eq(comb) {
                match op {
                    OP::MULT => val *= num,
                    OP::ADD => val += num,
                    _ => {}
                }
            }

            if val == eq[0] {
                // valid operation, add output number
                out += val;
                break;
            }
        }
    }

    out
}

fn part2(equations: &[Vec<u64>]) -> u64 {
    let mut out = 0;

    for eq in equations {
        // all operator combinations
        // 0 is add, 1 is multiply
        for comb in repeat_n([OP::MULT, OP::ADD, OP::CONC], eq.len() - 2).multi_cartesian_product()
        {
            // assign val to first number
            let mut val = eq[1];

            for (num, op) in eq.iter().skip(2).zip_eq(comb) {
                match op {
                    OP::MULT => val *= num,
                    OP::ADD => val += num,
                    OP::CONC => val = concact_num(val, *num),
                }
            }

            if val == eq[0] {
                // valid operation, add output number
                out += val;
                break;
            }
        }
    }

    out
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    let mut equations = vec![];
    for line in input.lines() {
        equations.push(
            line.split(|c: char| !c.is_numeric())
                .filter(|c| !c.is_empty())
                .map(|c| c.parse().expect("Error parsing number"))
                .collect(),
        );
    }

    equations
}

fn concact_num(n1: u64, n2: u64) -> u64 {
    format!("{n1}{n2}")
        .parse()
        .expect("unable to parse contacted number")
}

#[derive(Clone, Copy)]
enum OP {
    MULT,
    ADD,
    CONC,
}
