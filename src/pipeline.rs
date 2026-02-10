use anyhow::Result;
use std::fs::File;
use std::io::Write;

use crate::{
    blocklist::load_reference_dir,
    config::Config,
    ftl::load_domain_stats,
    intersect::intersect,
    path::Paths,
    state::RunState,
};

pub fn run(paths: &Paths, config: &Config) -> Result<()> {
    let stats = load_domain_stats(&paths.ftl_db)?;
    let reference = load_reference_dir(&paths.reference_dir)?;

    let selected = intersect(stats, reference);

    // Write managed blocklist
    {
        let mut file = File::create(&paths.managed_list)?;
        for d in &selected {
            writeln!(file, "{d}")?;
        }
    }

    let state = RunState::from_selection(&selected);
    state.write(&paths.state)?;

    if config.output.auto_reload {
        reload_pihole();
    }

    Ok(())
}

fn reload_pihole() {
    let _ = std::process::Command::new("pihole")
        .args(["restartdns", "reload-lists"])
        .status();
}
