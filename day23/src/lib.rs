use std::collections::HashSet;

use itertools::Itertools;

pub fn solve(input: &str) -> (usize, String) {
    let (connections, computers) = parse(input);

    (
        part1(&connections, &computers),
        part2(&connections, &computers),
    )
}

fn part1(connections: &HashSet<Connection<&str>>, computers: &HashSet<&str>) -> usize {
    let mut triplets = HashSet::new();
    for Connection(c1, c2) in connections {
        for c3 in computers {
            if c3 == c1 || c3 == c2 {
                continue;
            }

            if connections.contains(&Connection::new(c1, c3))
                && connections.contains(&Connection::new(c2, c3))
            {
                triplets.insert(Triplet::new(c1, c2, c3));
            }
        }
    }

    let mut p1 = 0;
    for Triplet(c1, c2, c3) in triplets {
        if c1.starts_with('t') || c2.starts_with('t') || c3.starts_with('t') {
            p1 += 1;
        }
    }

    p1
}

fn part2(connections: &HashSet<Connection<&str>>, computers: &HashSet<&str>) -> String {
    let mut party_size = 2;
    let mut largest_party = vec![];
    'size: loop {
        println!("Party Size of {party_size}");
        'party: for party in computers.iter().combinations(party_size) {
            // loop through all connections
            for c1 in 0..party.len() {
                for c2 in (c1 + 1)..party.len() {
                    // not a party if one connection is bad
                    if !connections.contains(&Connection::new(party[c1], party[c2])) {
                        continue 'party;
                    }
                }
            }
            // if all connections is fine
            largest_party = party;
            party_size += 1;
            continue 'size;
        }
        // if no party of the size is found, break
        break;
    }

    largest_party.sort();
    let mut output = String::new();
    for computer in largest_party {
        output.push_str(computer);
        output.push(',');
    }
    output.pop();
    output
}

fn parse(input: &str) -> (HashSet<Connection<&str>>, HashSet<&str>) {
    let mut computers = HashSet::new();
    let connections = input
        .lines()
        .map(|line| {
            let mut comp = line.split('-');
            let c1 = comp.next().expect("No first computer!");
            let c2 = comp.next().expect("No second computer!");
            computers.insert(c1);
            computers.insert(c2);
            Connection::new(c1, c2)
        })
        .collect();

    (connections, computers)
}

// to find 3 sets
// for every set (a,b)
// and every other computer c
// if a,c and b,c exist, then there is a connection

#[derive(Hash, PartialEq, Eq)]
struct Connection<T: Ord>(T, T);

impl<T: Ord> Connection<T> {
    fn new(a: T, b: T) -> Connection<T> {
        if a == b {
            panic!("Connection points can't be the same!")
        }
        if a > b {
            Connection(a, b)
        } else {
            Connection(b, a)
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Triplet<T: Ord>(T, T, T);

impl<T: Ord> Triplet<T> {
    fn new(a: T, b: T, c: T) -> Triplet<T> {
        if a == b || b == c || a == c {
            panic!("Triplet computers can't be the same!")
        }

        let mut vec = vec![a, b, c];
        vec.sort();

        Triplet(vec.pop().unwrap(), vec.pop().unwrap(), vec.pop().unwrap())
    }
}
