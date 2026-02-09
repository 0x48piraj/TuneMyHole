use std::{
    collections::HashSet,
    path::PathBuf,
};

use anyhow::Result;
use rusqlite::Connection;

use crate::domain::normalize_domain;

pub fn load_allowed_domains(db: &PathBuf) -> Result<HashSet<Box<str>>> {
    let conn = Connection::open(db)?;

    let mut stmt = conn.prepare(
        r#"
        SELECT DISTINCT domain
        FROM queries
        WHERE domain IS NOT NULL
          AND status = 0
        "#,
    )?;

    let rows = stmt.query_map([], |row| {
        let domain: String = row.get(0)?;
        Ok(domain)
    })?;

    let mut set = HashSet::new();

    for row in rows {
        let domain = row?;
        if let Some(d) = normalize_domain(&domain) {
            set.insert(d);
        }
    }

    Ok(set)
}
