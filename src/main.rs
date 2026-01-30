use std::process::exit;

use clap::Parser;
use cornelli::{
    cli::{Args, args::Command},
    commands::Runnable,
    core::ChristmasDB,
    log_err, log_orb,
    utils::io::clear_terminal,
};
use rpassword::prompt_password;

fn main() {
    let args = Args::parse();

    clear_terminal();
    log_orb!("Print your convenient, little password.");

    let password = match prompt_password("Put here (invisible):") {
        Ok(some) => some,
        Err(e) => {
            log_err!("Failed to grab password: {e}");
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
    };

    if let Err(err) = result {
        log_err!("{err}");
        exit(1);
    }
}
