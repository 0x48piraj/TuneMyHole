use std::collections::HashSet;
use crate::ftl::DomainStats;

pub fn intersect(
    stats: Vec<DomainStats>,
    reference: HashSet<Box<str>>,
) -> Vec<Box<str>> {
    let mut out: Vec<Box<str>> = stats
        .into_iter()
        .filter(|d| reference.contains(&d.domain))
        .map(|d| d.domain)
        .collect();

    out.sort();
    out
}
