use std::process::exit;

use clap::Parser;
use cornelli::{
    cli::{Args, args::Command},
    commands::Runnable,
    core::ChristmasDB,
    log_err,
};

fn main() {
    let args = Args::parse();

    let mut db = match ChristmasDB::init() {
        Ok(db) => db,
        Err(e) => {
            log_err!("ChristmasDB failed to initialize: {e}");
            exit(1);
        }
    };

    // command invocation
    let result = match &args.command {
        Command::Keep(cmd) => cmd.run(&mut db),
        Command::Burn(cmd) => cmd.run(&mut db),
        Command::Mailbox(cmd) => cmd.run(&mut db),
    };

    if let Err(err) = result {
        log_err!("{err}");
        exit(1);
    }
}
