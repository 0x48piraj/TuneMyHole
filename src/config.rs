use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub analysis: Analysis,
    pub output: Output,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub window_days: u32,
    pub min_queries: u32,
    pub target_domains: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub auto_reload: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            analysis: Analysis {
                window_days: 7,
                min_queries: 5,
                target_domains: 5_000,
            },
            output: Output {
                auto_reload: true,
            },
        }
    }
}

impl Config {
    pub fn load_or_default(path: &Path) -> Self {
        if let Ok(data) = fs::read_to_string(path) {
            toml::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn write_default(path: &Path) -> std::io::Result<()> {
        let cfg = toml::to_string_pretty(&Self::default()).unwrap();
        fs::write(path, cfg)
    }
}
