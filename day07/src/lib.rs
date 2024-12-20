use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::repeat_n,
    sync::Arc,
    thread,
};

pub fn solve(input: &str, nthreads: usize) -> (u64, u64) {
    let equations = parse(input);
    if nthreads == 0 {
        panic!("Can't have 0 threads!");
    }

    (
        part1_threaded(&equations, nthreads),
        part2_threaded(&equations, nthreads),
    )
}

fn part1_threaded(equations: &[Vec<u64>], nthreads: usize) -> u64 {
    // get all equation lengths
    let eq_lengths: HashSet<usize> = equations.iter().map(|eq| eq.len()).collect();

    // precompile all permutations
    let combs: Arc<HashMap<usize, Vec<Vec<OP>>>> = Arc::new(
        eq_lengths
            .into_iter()
            .map(|len| {
                (
                    len,
                    repeat_n([OP::ADD, OP::MULT], len - 2)
                        .multi_cartesian_product()
                        .collect(),
                )
            })
            .collect(),
    );

    let chunk_size = if nthreads > equations.len() {
        equations.len()
    } else {
        equations.len() / nthreads
    };

    // create threadpool
    let threads: Vec<_> = equations
        .chunks(chunk_size)
        .map(|chunk| {
            let combs = combs.clone();
            let chunk = chunk.to_vec();
            thread::spawn(move || part1(&(chunk.to_vec()), &combs))
        })
        .collect();

    threads
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

fn part2_threaded(equations: &[Vec<u64>], nthreads: usize) -> u64 {
    // get all equation lengths
    let eq_lengths: HashSet<usize> = equations.iter().map(|eq| eq.len()).collect();

    // precompile all permutations
    let combs: Arc<HashMap<usize, Vec<Vec<OP>>>> = Arc::new(
        eq_lengths
            .into_iter()
            .map(|len| {
                (
                    len,
                    repeat_n([OP::ADD, OP::MULT, OP::CONC], len - 2)
                        .multi_cartesian_product()
                        .collect(),
                )
            })
            .collect(),
    );

    let chunk_size = if nthreads > equations.len() {
        equations.len()
    } else {
        equations.len() / nthreads
    };

    // create threadpool
    let threads: Vec<_> = equations
        .chunks(chunk_size)
        .map(|chunk| {
            let combs = combs.clone();
            let chunk = chunk.to_vec();
            thread::spawn(move || part2(&(chunk.to_vec()), &combs))
        })
        .collect();

    threads
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

fn part1(equations: &[Vec<u64>], combs: &HashMap<usize, Vec<Vec<OP>>>) -> u64 {
    let mut out = 0;

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

fn part2(equations: &[Vec<u64>], combs: &HashMap<usize, Vec<Vec<OP>>>) -> u64 {
    let mut out = 0;

    for eq in equations {
        // all operator permutations with replacement
        let combs = combs.get(&eq.len()).expect("Invaid equation length");

        'comb_test: for comb in combs {
            // assign first number to val
            let mut val = eq[1];

            for (num, op) in eq.iter().skip(2).zip_eq(comb) {
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
    n1 * 10u64.pow(n2.ilog10() + 1) + n2
}

#[derive(Clone, Copy, PartialEq)]
enum OP {
    MULT,
    ADD,
    CONC,
}
