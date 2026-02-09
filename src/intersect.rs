use std::collections::HashSet;

pub fn intersect(
    allowed: HashSet<Box<str>>,
    blocklist: HashSet<Box<str>>,
) -> Vec<Box<str>> {
    let mut out: Vec<Box<str>> = allowed
        .into_iter()
        .filter(|d| blocklist.contains(d))
        .collect();

    out.sort();
    out
}
