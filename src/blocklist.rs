use std::{
    collections::HashSet,
    fs::File,
    fs::read_dir,
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

pub fn load_reference_dir(dir: &PathBuf) -> Result<HashSet<Box<str>>> {
    let mut out = HashSet::new();

    if !dir.exists() {
        return Ok(out);
    }

    for entry in read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            out.extend(load_blocklist(&path)?);
        }
    }

    Ok(out)
}
