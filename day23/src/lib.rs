use hashbrown::{HashMap, HashSet};

#[cfg(test)]
mod tests;

pub fn solve(input: &str) -> (usize, String) {
    let (connections, computers) = parse(input);

    (
        part1(&connections, &computers),
        find_largest_party(&connections, &computers),
    )
}

fn part1(connections: &HashSet<Connection<&str>>, computers: &HashSet<&str>) -> usize {
    let mut triplets = HashSet::new();
    for Connection(c1, c2) in connections {
        for c3 in computers {
            if c3 == c1 || c3 == c2 {
                continue;
            }

            if connections.contains(&Connection::new(*c1, *c3))
                && connections.contains(&Connection::new(*c2, *c3))
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

fn find_largest_party(
    connections: &HashSet<Connection<&str>>,
    computers: &HashSet<&str>,
) -> String {
    let mut parties = HashMap::new();

    // create new clique that has possible further additions
    // excludes itself
    for pc in computers {
        let mut possibles = computers.clone();
        possibles.remove(pc);
        parties.insert(vec![*pc], possibles);
    }

    loop {
        let mut larger_party = HashMap::new();
        // goes through every previously valid party
        for (party, possibles) in &parties {
            let mut passing = vec![];
            let mut new_possibles = possibles.clone();

            // adds another pc if it is unique
            'new_pc: for new_pc in possibles {
                if party.contains(new_pc) {
                    panic!("shouldnt happen");
                }

                // checks if all previous pcs have connections to current one
                // previous pc are guarenteed to connect to each other already
                for old_pc in party {
                    if !connections.contains(&Connection::new(*old_pc, *new_pc)) {
                        new_possibles.remove(new_pc);
                        continue 'new_pc;
                    }
                }

                // is in party!
                passing.push(*new_pc);
            }

            for new_pc in passing {
                let mut new_p = new_possibles.clone();
                new_p.remove(new_pc);

                let mut new_party = party.clone();
                new_party.push(new_pc);
                new_party.sort();

                if let Some(other_p) = larger_party.get_mut(&new_party) {
                    *other_p = merge_possibles(other_p, &new_p);
                } else {
                    // new_pc is connected to all old_pcs so party is valid
                    larger_party.insert(new_party, new_p);
                }
            }
        }

        // break if no larger party is found
        if larger_party.is_empty() {
            break;
        }
        parties = larger_party;
    }

    assert!(parties.len() == 1);

    let mut output = String::new();
    let (largest_party, _) = parties.iter().next().unwrap();
    for computer in largest_party {
        output.push_str(computer);
        output.push(',');
    }
    output.pop();
    output
}

fn merge_possibles<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut output = HashSet::new();

    for pc in a {
        if b.contains(pc) {
            output.insert(*pc);
        }
    }

    output
}

fn parse(input: &str) -> (HashSet<Connection<&str>>, HashSet<&str>) {
    let mut computers = HashSet::new();
    let connections = input
        .lines()
        .map(|line| {
            let mut pcs = line.split('-');
            let c1 = pcs.next().expect("No first computer!");
            let c2 = pcs.next().expect("No second computer!");
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
