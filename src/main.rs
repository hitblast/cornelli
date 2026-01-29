use std::process::exit;

use clap::Parser;
use cornelli::{
    cli::{Args, args::Command},
    commands::Runnable,
    core::ChristmasDB,
    log_err,
    utils::io::get_string_input,
};

fn main() {
    let args = Args::parse();

    let password = match get_string_input("Enter your convenient, little password.") {
        Ok(p) => p,
        Err(e) => {
            log_err!("{e}");
            exit(1);
        }
    };

    let mut db = match ChristmasDB::init(password) {
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
        Command::Path(cmd) => cmd.run(&mut db),
    };

    if let Err(err) = result {
        log_err!("{err}");
        exit(1);
    }
}
