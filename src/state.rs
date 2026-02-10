use serde::{Serialize, Deserialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct RunState {
    pub domains_blocked: usize,
    pub last_run: String,
    pub empty_reference: bool,
}

impl RunState {
    pub fn from_selection(domains: &[Box<str>], empty_reference: bool) -> Self {
        Self {
            domains_blocked: domains.len(),
            last_run: chrono::Utc::now().to_rfc3339(),
            empty_reference,
        }
    }

    pub fn write(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json)
    }

    pub fn load(path: &Path) -> Option<Self> {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
    }
}
