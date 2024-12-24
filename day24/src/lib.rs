use core::panic;
use std::collections::HashMap;

pub fn solve(input: &str) -> (u64, usize) {
    let (initials, gates) = parse(input);
    (part1(&initials, &gates), part2())
}

fn part1(initials: &HashMap<&str, bool>, gates: &[Gate]) -> u64 {
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

    let mut output = 0;
    for (gate, val) in values {
        if val && gate.starts_with('z') {
            output += 2u64.pow(gate[1..].parse().unwrap());
        }
    }

    output
}

fn part2() -> usize {
    0
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

#[derive(Clone)]
struct Gate<'a> {
    i1: &'a str,
    i2: &'a str,
    out: &'a str,
    logic: Logic,
}

#[derive(Clone, Copy)]
enum Logic {
    And,
    Or,
    Xor,
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn p1_small() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(solve(&input).0, 4)
    }

    #[test]
    fn p1_big() {
        let input = std::fs::read_to_string("example_big.txt").unwrap();
        assert_eq!(solve(&input).0, 2024)
    }
}
