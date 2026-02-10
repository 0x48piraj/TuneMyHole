use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    config::Config,
    path::Paths,
    pipeline,
    state::RunState,
    install::{init_system, uninstall_system},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Run,
    Status,
    Init,
    Uninstall,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let paths = Paths::detect();

    match cli.cmd {
        Command::Init => {
            init_system(&paths)?;
            println!("Tune My Hole installed and scheduled.");
        }
        Command::Uninstall => {
            uninstall_system(&paths)?;
            println!("Tune My Hole uninstalled.");
        }
        Command::Run => {
            let config = Config::load_or_default(&paths.config);
            pipeline::run(&paths, &config)?;
        }
        Command::Status => {
            if let Some(state) = RunState::load(&paths.state) {
                println!("Tune My Hole");
                println!("Blocked domains: {}", state.domains_blocked);
                println!("Last run: {}", state.last_run);

                if state.empty_reference {
                    println!();
                    println!("[!] No reference lists found.");
                    println!("    Drop blocklists into /etc/pihole/tune-my-hole.d/");
                }
            } else {
                println!("No runs yet.");
            }
        }
    }

    Ok(())
}
