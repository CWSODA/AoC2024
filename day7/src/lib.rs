use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::repeat_n,
};

pub fn solve(input: &str) -> (u64, u64) {
    let equations = parse(input);

    (part1(&equations), part2(&equations))
}

fn part1(equations: &[Vec<u64>]) -> u64 {
    let mut out = 0;

    let mut eq_lengths = HashSet::new();
    for eq in equations {
        eq_lengths.insert(eq.len());
    }
    let combs: HashMap<usize, Vec<Vec<OP>>> = eq_lengths
        .into_iter()
        .map(|len| {
            (
                len,
                repeat_n([OP::ADD, OP::MULT], len - 2)
                    .multi_cartesian_product()
                    .collect(),
            )
        })
        .collect();

    for eq in equations {
        // all operator permutations with replacement
        for comb in combs.get(&eq.len()).expect("Invalid equation length") {
            // assign first number to val
            let mut val = eq[1];

            for (num, op) in eq.iter().skip(2).zip_eq(comb) {
                match op {
                    OP::MULT => val *= num,
                    OP::ADD => val += num,
                    _ => {}
                }

                // val only increases with each operation
                if val > eq[0] {
                    break;
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

    let mut eq_lengths = HashSet::new();
    for eq in equations {
        eq_lengths.insert(eq.len());
    }
    let combs: HashMap<usize, Vec<Vec<OP>>> = eq_lengths
        .into_iter()
        .map(|len| {
            (
                len,
                repeat_n([OP::ADD, OP::MULT, OP::CONC], len - 2)
                    .multi_cartesian_product()
                    .collect(),
            )
        })
        .collect();

    for eq in equations {
        // all operator permutations with replacement
        let combs = combs.get(&eq.len()).expect("Invaid equation length");

        'comb_test: for comb in combs {
            // assign first number to val
            let mut val = eq[1];

            for (num, op) in eq.into_iter().skip(2).zip_eq(comb) {
                match op {
                    OP::MULT => val *= num,
                    OP::ADD => val += num,
                    OP::CONC => val = concact_num(val, *num),
                }

                // val only increases with each operation so stop current comb test
                if val > eq[0] {
                    continue 'comb_test;
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
    n1 * 10u64.pow(n2.to_string().len() as u32) + n2
}

#[derive(Clone, Copy, PartialEq)]
enum OP {
    MULT,
    ADD,
    CONC,
}

// 3749, 11387
// 5702958180383, 92612386119138
//92612386119138
//30383733113065

/*
fn part2(equations: &[Vec<u64>]) -> u64 {
    let mut out = 0;

    for eq in equations {
        // all operator permutations with replacement
        'comb: for comb in
            repeat_n([OP::ADD, OP::MULT, OP::CONC], eq.len() - 2).multi_cartesian_product()
        {
            // assign first number to val
            let mut val = eq[1];

            for (num, op) in eq.iter().skip(2).zip_eq(comb) {
                match op {
                    OP::MULT => val *= num,
                    OP::ADD => val += num,
                    OP::CONC => val = concact_num(val, *num),
                }

                // val only increases with each operation
                if val > eq[0] {
                    continue 'comb;
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
*/
