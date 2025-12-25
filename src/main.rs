use std::{env, path::Path, process::exit};

use clap::Parser;
use cornelli::{
    cli::{Args, args::Command, atomic::set_accept_all},
    commands::Runnable,
    core::ChristmasDB,
    log_err,
    utils::sudo::run_with_noroot,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();

    // sudo protection
    if let Err(err) = run_with_noroot() {
        log_err!("{err}");
        exit(1);
    }

    // set global flags' values
    set_accept_all(args.accept_all);

    // create the database
    let pass = match env::var("CORNELLI_PASS") {
        Ok(var) => var,
        Err(_) => {
            log_err!("Password wasn't provided with CORNELLI_PASS.");
            exit(1)
        }
    };

    let mut db =
        match ChristmasDB::load_from_pass(pass, Path::new("database.json").to_path_buf()).await {
            Ok(db) => db,
            Err(_) => {
                log_err!("ChristmasDB failed to initialize.");
                exit(1);
            }
        };

    // command invocation
    let result = match &args.command {
        Command::Keep(cmd) => cmd.run(&mut db).await,
        Command::Mailbox(cmd) => cmd.run(&mut db).await,
    };

    if let Err(err) = result {
        log_err!("{err}");
        exit(1);
    }
}
