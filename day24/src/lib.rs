use core::panic;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: &str, nbad_pairs: Option<usize>) -> (u64, String) {
    let (initials, gates) = parse(input);

    let p1 = run_sim(&initials, &gates);

    let p2 = if let Some(nbad_pairs) = nbad_pairs {
        bad_gates(&initials, &gates, nbad_pairs)
    } else {
        String::new()
    };

    (p1, p2)
}

fn run_sim<'a>(initials: &HashMap<&'a str, bool>, gates: &[Gate<'a>]) -> u64 {
    let mut gates = gates.to_vec();
    let mut values = initials.clone();

    while gates.len() != 0 {
        for i in 0..gates.len() {
            let gate = &gates[i];

            // check if both inputs are valid yet
            if let (Some(&i1), Some(&i2)) = (values.get(gate.i1), values.get(gate.i2)) {
                values.insert(
                    gate.out,
                    match gate.logic {
                        Logic::And => i1 && i2,
                        Logic::Or => i1 || i2,
                        Logic::Xor => i1 ^ i2,
                    },
                );
                gates.remove(i);
                break;
            }
        }
    }

    gate_value('z', &values)
}

fn bad_gates(initials: &HashMap<&str, bool>, gates: &[Gate], nbad_pairs: usize) -> String {
    let sum = gate_value('x', &initials) + gate_value('y', &initials);

    for bad_pairs in gates
        .iter()
        .tuple_combinations::<(_, _)>()
        .combinations(nbad_pairs)
    {
        // checks if the pairs have all unique gates
        let mut seen = HashSet::new();
        for bad_pair in &bad_pairs {
            seen.insert(bad_pair.0);
            seen.insert(bad_pair.1);
        }
        if seen.len() != nbad_pairs * 2 {
            continue;
        }

        // creates list of gates
        let mut changed_gates = gates
            .iter()
            .filter(|g| !seen.contains(g))
            .cloned()
            .collect::<Vec<_>>();

        // create the new gates with swapped outputs
        let mut changed_outs = vec![];
        for (g1, g2) in bad_pairs {
            changed_gates.push(Gate {
                i1: g1.i1,
                i2: g1.i2,
                out: g2.out,
                logic: g1.logic,
            });
            changed_gates.push(Gate {
                i1: g2.i1,
                i2: g2.i2,
                out: g1.out,
                logic: g2.logic,
            });

            changed_outs.push(g1.out);
            changed_outs.push(g2.out);
        }

        let z = run_sim(initials, &changed_gates);

        println!("{sum} = {z}?");
        // checks if x+y = z
        if sum == z {
            // yay!!!
            changed_outs.sort();
            let mut ans = String::new();
            for out in changed_outs {
                ans += out;
                ans += ",";
            }

            ans.pop();
            return ans;
        }
    }

    unreachable!()
}

fn gate_value(ch: char, values: &HashMap<&str, bool>) -> u64 {
    let mut output = 0;
    for (gate, &val) in values {
        if val && gate.starts_with(ch) {
            output += 2u64.pow(gate[1..].parse().unwrap());
        }
    }
    output
}

fn parse(input: &str) -> (HashMap<&str, bool>, Vec<Gate>) {
    let mut initials = HashMap::new();
    for line in input.split("\n\n").next().unwrap().lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let value = match parts.next().unwrap() {
            "1" => true,
            "0" => false,
            _ => panic!("Invalid logic!"),
        };
        initials.insert(key, value);
    }

    let mut gates = vec![];
    for line in input.split("\n\n").nth(1).unwrap().lines() {
        let mut parts = line.split_whitespace();
        let i1 = parts.next().unwrap();
        let logic = match parts.next().unwrap() {
            "AND" => Logic::And,
            "OR" => Logic::Or,
            "XOR" => Logic::Xor,
            _ => panic!("Invalid logic type!"),
        };
        let i2 = parts.next().unwrap();
        let out = parts.skip(1).next().unwrap();

        gates.push(Gate { i1, i2, out, logic });
    }

    (initials, gates)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Gate<'a> {
    i1: &'a str,
    i2: &'a str,
    out: &'a str,
    logic: Logic,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Logic {
    And,
    Or,
    Xor,
}

#[cfg(test)]
mod tests;
