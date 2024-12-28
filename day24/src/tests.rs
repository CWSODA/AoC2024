use super::*;

#[test]
fn p1_small() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    assert_eq!(solve(&input, None).0, 4)
}

#[test]
fn p1_big() {
    let input = std::fs::read_to_string("example_big.txt").unwrap();
    assert_eq!(solve(&input, None).0, 2024)
}

#[test]
fn p2() {
    let input = std::fs::read_to_string("example_p2.txt").unwrap();
    assert_eq!(solve(&input, Some(2)).1, "z00,z01,z02,z05")
}
