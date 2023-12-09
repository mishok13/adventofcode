pub fn parse_space_separated_str(s: &str) -> Vec<i128> {
    s.split(' ').flat_map(|s| s.parse().ok()).collect()
}
