use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;

use crate::domain::parse_gravity_line;

pub fn load_blocklist(path: &PathBuf) -> Result<HashSet<Box<str>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut set = HashSet::new();

    for line in reader.lines() {
        if let Some(domain) = parse_gravity_line(&line?) {
            set.insert(domain);
        }
    }

    Ok(set)
}
