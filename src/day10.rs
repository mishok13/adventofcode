pub fn part1(lines: Vec<String>) -> Option<String> {
    None
}

pub fn part2(lines: Vec<String>) -> Option<String> {
    None
}

#[test]
fn test_part1() {
    let lines = "".lines().map(String::from).collect();
    assert_eq!(part1(lines), None);
}

#[test]
fn test_part2() {
    let lines = "";
    assert_eq!(part2(lines.lines().map(String::from).collect()), None);
}
