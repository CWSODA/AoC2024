use super::*;

#[test]
fn test1() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let (p1, p2) = solve(&input);
    assert_eq!(p1, 7);
    assert_eq!(p2, "co,de,ka,ta");
}
