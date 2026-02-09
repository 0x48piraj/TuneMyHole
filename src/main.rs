use std::{
    fs::File,
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

use tmhole::{
    blocklist::load_blocklist,
    ftl::load_allowed_domains,
    intersect::intersect,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to Pi-hole FTL database
    #[arg(long)]
    db: PathBuf,

    /// External reference blocklist (OISD, HaGeZi, etc.)
    #[arg(long)]
    blocklist: PathBuf,

    /// Output file
    #[arg(long)]
    output: PathBuf,

    /// Print results without writing file
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let allowed = load_allowed_domains(&args.db)?;
    let blocklist = load_blocklist(&args.blocklist)?;

    let selected = intersect(allowed, blocklist);

    if args.dry_run {
        for d in &selected {
            println!("{d}");
        }
    } else {
        let mut file = File::create(&args.output)?;
        for d in &selected {
            writeln!(file, "{d}")?;
        }
    }

    Ok(())
}
